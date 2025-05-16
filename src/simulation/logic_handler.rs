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

fn propagate_wires(voxel_map: &VoxelMap) -> Vec<LogicEvent> {
    let mut events = Vec::new();
    let mut visited: HashSet<IVec3> = HashSet::new();

    // 1) collect every gate-output position → its full Bits16 word
    let mut active_outputs: HashMap<IVec3, Bits16> = HashMap::new();
    for (&pos, voxel) in voxel_map.voxel_map.iter() {
        // skip wires
        if let VoxelType::Wire(_) = voxel.kind {
            continue;
        }
        // if any bit is set, propagate the entire word to its output cell
        if voxel.state.any_set() {
            let (_, output_pos) = voxel_directions(voxel);
            active_outputs
                .entry(output_pos)
                .and_modify(|word| {
                    // OR together in case multiple gates drive the same spot
                    *word = Bits16::new(word.value() | voxel.state.value())
                })
                .or_insert(voxel.state);
        }
    }

    // 2) six-way neighbor offsets for flood-fill
    let directions = [
        IVec3::new(1, 0, 0),
        IVec3::new(-1, 0, 0),
        IVec3::new(0, 1, 0),
        IVec3::new(0, -1, 0),
        IVec3::new(0, 0, 1),
        IVec3::new(0, 0, -1),
    ];

    // 3) for each unvisited wire-voxel, flood-fill its connected component
    for (&start_pos, start_voxel) in voxel_map.voxel_map.iter() {
        if visited.contains(&start_pos) {
            continue;
        }

        // only proceed if this is a Wire(channel)
        let channel = if let VoxelType::Wire(ch) = start_voxel.kind {
            ch
        } else {
            continue;
        };

        let mut queue = VecDeque::new();
        let mut component = Vec::new();
        visited.insert(start_pos);
        queue.push_back(start_pos);

        while let Some(cur) = queue.pop_front() {
            component.push(cur);
            for &d in &directions {
                let nb = cur + d;
                if visited.contains(&nb) {
                    continue;
                }
                if let Some(nb_voxel) = voxel_map.voxel_map.get(&nb) {
                    if nb_voxel.kind == VoxelType::Wire(channel) {
                        visited.insert(nb);
                        queue.push_back(nb);
                    }
                }
            }
        }

        // 4) decide whether *this* channel should be on anywhere in the component
        let desired_on = component.iter().any(|&p| {
            active_outputs
                .get(&p)
                .map_or(false, |word| word.get(channel))
        });

        // 5) for each wire-voxel in that component, if its bit differs, schedule an update
        for &p in &component {
            if let Some(wire_voxel) = voxel_map.voxel_map.get(&p) {
                let currently_on = wire_voxel.state.get(channel);
                if currently_on != desired_on {
                    // build the new 16-bit word: only this channel bit
                    let mut new_word = Bits16::all_zeros();
                    if desired_on {
                        new_word.set(channel);
                    }
                    events.push(LogicEvent::UpdateVoxel {
                        position: p,
                        new_state: new_word,
                    });
                }
            }
        }
    }

    events
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
