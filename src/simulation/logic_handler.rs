use std::collections::{HashSet, VecDeque};

use bevy::{reflect::{Map, Set}, utils::HashMap};

use crate::prelude::*; 

#[derive(Resource)]
pub struct SimulationResource {
    pub tick_timer: Timer,
    pub dirty_voxels: HashSet<IVec3>,
}

#[derive(Default, Resource)]
pub struct Scratch {
    pub queue:    VecDeque<IVec3>,
    pub visited:  bevy::utils::HashSet<IVec3>,
    pub outputs:  bevy::utils::HashSet<IVec3>,
}


#[derive(Event, Debug)]
pub enum LogicEvent {
    Skip, // Event flag that skips the current iteration of the event handler
    UpdateVoxel {
        position: IVec3,
        new_state: Bits16,
    },
}

pub fn logic_event_handler(
    mut logic_events: EventReader<LogicEvent>,
    mut voxel_map: ResMut<VoxelMap>,
    mut commands: Commands,
) {
    for event in logic_events.read() {
        match event {
            LogicEvent::Skip => {
                // No-op for skip events.
            }
            LogicEvent::UpdateVoxel { position, new_state } => {
                // First, immutably borrow the entity from the entity_map.
                // This borrow is dropped after this block.
                if let Some(entity) = voxel_map.entity_map.get(&position).cloned() {
                    // Now borrow the voxel map mutably.
                    if let Some(voxel) = voxel_map.voxel_map.get_mut(&position) {
                        // Update only if the state is different.
                        if voxel.state != *new_state {
                            voxel.state = *new_state;
                            //println!("Changed voxel at {:?}: {:?}", position, voxel);
                            commands.entity(entity).insert(voxel.clone());
                        }
                    }
                }
            }
        }
    }
}

pub fn logic_system(
    time: Res<Time>,
    mut simulation_resource: ResMut<SimulationResource>,
    voxel_map: ResMut<VoxelMap>,
    mut logic_writer: EventWriter<LogicEvent>,
) {
    simulation_resource.tick_timer.tick(time.delta());

    if simulation_resource.tick_timer.finished() {
        // On the first tick or if no voxels are marked dirty, mark all voxels as dirty.
        if simulation_resource.dirty_voxels.is_empty() {
            simulation_resource
                .dirty_voxels
                .extend(voxel_map.voxel_map.keys().cloned());
        }

        let mut next_dirty = HashSet::new();

        // Process only voxels marked as dirty.
        for pos in &simulation_resource.dirty_voxels {
            if let Some(voxel) = voxel_map.voxel_map.get(pos) {
                if let Some(new_state) = simulate_gate(voxel, &voxel_map) {
                    // Update the gate voxel.
                    logic_writer.send(LogicEvent::UpdateVoxel {
                        position: *pos,
                        new_state,
                    });
                    // Mark the output voxel as dirty for the next tick.
                    let (_, output) = voxel_directions(voxel);
                    if let Some(output_voxel) = voxel_map.voxel_map.get(&output) {
                        match output_voxel.kind {
                            VoxelType::Wire(_) => {
                                logic_writer.send(LogicEvent::UpdateVoxel {
                                    position: output,
                                    new_state,
                                });
                            }
                            _ => {}
                        }
                    }
                    next_dirty.insert(output);
                }
            }
        }

        // Get the propagation events for wires (turning them on or off as needed)
        let wire_events = propagate_wires(&voxel_map);
        for event in wire_events {
            logic_writer.send(event);
        }

        // Prepare the dirty set for the next simulation tick.
        simulation_resource.dirty_voxels = next_dirty;
    }
}

#[inline]
fn carries(voxel: &Voxel, channel: u8) -> bool {
    match voxel.kind {
        VoxelType::Wire(ch)        => ch == channel,
        VoxelType::BundledWire     => true,
        _                          => false,
    }
}

pub fn propagate_wires(voxel_map: &VoxelMap) -> Vec<LogicEvent> {
    use std::collections::{HashMap, HashSet, VecDeque};

    // --- 1. gather every *gate* output word ---------------------------------
    let mut gate_drive: HashMap<IVec3, Bits16> = HashMap::new();

    for (&pos, v) in &voxel_map.voxel_map {
        if matches!(v.kind, VoxelType::Wire(_) | VoxelType::BundledWire) {
            continue;                       // skip cables themselves
        }
        if v.state.any_set() {
            let (_, out_pos) = voxel_directions(v);
            gate_drive
                .entry(out_pos)
                .and_modify(|w| *w = Bits16::new(w.value() | v.state.value()))
                .or_insert(v.state);
        }
    }

    // --- 2. flood‑fill *per channel* ----------------------------------------
    let dirs = [
        IVec3::new( 1, 0, 0), IVec3::new(-1, 0, 0),
        IVec3::new( 0, 1, 0), IVec3::new( 0,-1, 0),
        IVec3::new( 0, 0, 1), IVec3::new( 0, 0,-1),
    ];

    // we collect all pending edits first, so a BundledWire can get
    // several bits flipped in the same tick without races
    let mut pending: HashMap<IVec3, Bits16> = HashMap::new();

    // channels are 1‑based in your API
    for ch in 0..16u8 {
        let mut visited: HashSet<IVec3> = HashSet::new();

        for (&start_pos, start_v) in &voxel_map.voxel_map {
            if visited.contains(&start_pos) || !carries(start_v, ch) {
                continue;
            }

            // ----- breadth‑first search over carriers of *this* channel -----
            let mut queue     = VecDeque::new();
            let mut component = Vec::new();

            visited.insert(start_pos);
            queue.push_back(start_pos);

            while let Some(cur) = queue.pop_front() {
                component.push(cur);

                for &d in &dirs {
                    let nb = cur + d;
                    if visited.contains(&nb) {
                        continue;
                    }
                    if let Some(nb_voxel) = voxel_map.voxel_map.get(&nb) {
                        if carries(nb_voxel, ch) {
                            visited.insert(nb);
                            queue.push_back(nb);
                        }
                    }
                }
            }

            // ----- does anything in this blob *want* the bit on? ------------
            let driven_high = component.iter().any(|&p| {
                gate_drive
                    .get(&p)
                    .map_or(false, |w| w.get(ch))
            });

            // ----- schedule updates where the bit differs -------------------
            for &p in &component {
                let cur_word = voxel_map.voxel_map[&p].state;
                let bit_is_on = cur_word.get(ch);

                if bit_is_on != driven_high {
                    pending
                        .entry(p)
                        .and_modify(|w| {
                            // might already contain other channel edits
                            if driven_high { w.set(ch) } else { w.clear(ch) }
                        })
                        .or_insert_with(|| {
                            let mut w = cur_word;
                            if driven_high { w.set(ch) } else { w.clear(ch) }
                            w
                        });
                }
            }
        }
    }

    // --- 3. convert the accumulated edits into LogicEvents ------------------
    pending
        .into_iter()
        .map(|(pos, ns)| LogicEvent::UpdateVoxel { position: pos, new_state: ns })
        .collect()
}

fn simulate_gate(voxel: &Voxel, voxels: &VoxelMap) -> Option<Bits16> {
    // --- gather the two logical inputs (boolean) ---------------------------
    let (ins, _) = voxel_directions(voxel);
    let mut in_sig = [false; 2];

    for (slot, &pos) in ins.iter().take(2).enumerate() {
        in_sig[slot] = voxels
            .voxel_map
            .get(&pos)
            .map_or(false, |v| v.state.any_set());
    }

    use VoxelType::*;
    use AndVariants::*;
    use OrVariants::*;
    use XorVariants::*;
    use NotVariants::*;
    use LatchVariants::*;

    let out_bool = match voxel.kind {
        Not(NotGate)            => !in_sig[0],
        Not(BufferGate)         =>  in_sig[0],

        And(AndGate)            =>  in_sig[0]  &  in_sig[1],
        And(NandGate)           => !(in_sig[0]  &  in_sig[1]),

        Or(OrGate)              =>  in_sig[0]  |  in_sig[1],
        Or(NorGate)             => !(in_sig[0]  |  in_sig[1]),

        Xor(XorGate)            =>  in_sig[0]  ^  in_sig[1],
        Xor(XnorGate)           => !(in_sig[0]  ^  in_sig[1]),

        Latch(DFlipFlop) => {
            let d   = in_sig[0];
            let clk = in_sig[1];
            if clk { d } else { voxel.state.any_set() }
        }

        _ => return None,   // voxels that aren’t logic gates
    };

    let new_state = bitword(out_bool);

    (new_state != voxel.state).then_some(new_state)
}

pub fn voxel_directions(voxel: &Voxel) -> (Vec<IVec3>, IVec3) {
    let mut inputs = Vec::new();
    // Use IVec3::ZERO if available; otherwise, use IVec3::new(0, 0, 0)
    let mut output = IVec3::new(0, 0, 0);
    let position = voxel.position;
    let is_single_input = 
        voxel.kind == VoxelType::Not(NotVariants::BufferGate) 
        || voxel.kind == VoxelType::Not(NotVariants::NotGate);

    match voxel.direction {
        1 => {
            if is_single_input {
                // For not gates, the input is on the front and output on the back.
                inputs.push(position + IVec3::new(0, 0, -1));
                output = position + IVec3::new(0, 0, 1);
            } else {
                // For standard gates, the inputs are on the left and right.
                inputs.push(position + IVec3::new(1, 0, 0));
                inputs.push(position + IVec3::new(-1, 0, 0));
                output = position + IVec3::new(0, 0, 1);
            }
        }
        2 => {
            if is_single_input {
                inputs.push(position + IVec3::new(-1, 0, 0));
                output = position + IVec3::new(1, 0, 0);
            } else {
                inputs.push(position + IVec3::new(0, 0, -1));
                inputs.push(position + IVec3::new(0, 0, 1));
                output = position + IVec3::new(1, 0, 0);
            }
        }
        3 => {
            if is_single_input {
                inputs.push(position + IVec3::new(0, 0, 1));
                output = position + IVec3::new(0, 0, -1);
            } else {
                inputs.push(position + IVec3::new(-1, 0, 0));
                inputs.push(position + IVec3::new(1, 0, 0));
                output = position + IVec3::new(0, 0, -1);
            }
        }
        4 => {
            if is_single_input {
                inputs.push(position + IVec3::new(1, 0, 0));
                output = position + IVec3::new(-1, 0, 0);
            } else {
                inputs.push(position + IVec3::new(0, 0, 1));
                inputs.push(position + IVec3::new(0, 0, -1));
                output = position + IVec3::new(-1, 0, 0);
            }
        }
        _ => {
            // Consider using a Result or Option type here to propagate errors
            println!("!!!! Invalid direction");
        }
    }
    (inputs, output)
}
