use std::collections::{HashSet, VecDeque};

use bevy::reflect::{Map, Set};

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
        new_state: bool,
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

    // Build a set of active outputs: for each non-wire voxel (e.g. logic gate) that is on,
    // compute its output position and add it to the set.
    let mut active_outputs: HashSet<IVec3> = HashSet::new();
    for (&pos, voxel) in voxel_map.voxel_map.iter() {
        match voxel.kind {
            VoxelType::Wire(_) => {continue;}
            _ => {
                if voxel.state {
                    // Compute the output position using your helper.
                    let (_, output) = voxel_directions(voxel);
                    active_outputs.insert(output);
                }
            }
        }
    }

    // Directions for the 6 adjacent neighbors.
    let directions = [
        IVec3::new(1, 0, 0),
        IVec3::new(-1, 0, 0),
        IVec3::new(0, 1, 0),
        IVec3::new(0, -1, 0),
        IVec3::new(0, 0, 1),
        IVec3::new(0, 0, -1),
    ];

    // Iterate over all wire voxels.
    for (&pos, voxel) in voxel_map.voxel_map.iter() {
        // Only consider wires and skip if already visited.
        if visited.contains(&pos) { continue; }
    
        // ► skip anything that is *not* a wire
        if !matches!(voxel.kind, VoxelType::Wire(_)) {
            continue;
        }

        // Record the type of wire for this connected component.
        let wire_type = voxel.kind;

        // Found an unvisited wire voxel; perform BFS to collect the entire connected component,
        // but only consider wires of the same type.
        let mut component = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back(pos);
        visited.insert(pos);

        while let Some(current) = queue.pop_front() {
            component.push(current);
            for &dir in &directions {
                let neighbor = current + dir;
                if visited.contains(&neighbor) {
                    continue;
                }
                if let Some(neighbor_voxel) = voxel_map.voxel_map.get(&neighbor) {
                    // Only add neighbors that are wires and have the same type.
                    if neighbor_voxel.kind == wire_type {
                        queue.push_back(neighbor);
                        visited.insert(neighbor);
                    }
                }
            }
        }

        // Only activate this component if one of its wires is exactly at an active output.
        let desired_state = component.iter().any(|&pos| active_outputs.contains(&pos));

        // For each wire voxel in the component, if its state does not match the desired state,
        // record an update event.
        for pos in component {
            if let Some(voxel) = voxel_map.voxel_map.get(&pos) {
                if voxel.state != desired_state {
                    events.push(LogicEvent::UpdateVoxel {
                        position: pos,
                        new_state: desired_state,
                    });
                }
            }
        }
    }
    events
}

fn simulate_gate(voxel: &Voxel, voxels: &VoxelMap) -> Option<bool> {
    let (ins, _) = voxel_directions(voxel);

    // tiny stack buffer (max 2 inputs in your design)
    let mut in_state = [false; 2];
    for (slot, &p) in ins.iter().take(2).enumerate() {
        in_state[slot] = voxels.voxel_map.get(&p).map_or(false, |v| v.state);
    }

    use VoxelType::*;
    let ns = match voxel.kind {
        Not(NotVariants::NotGate)     => !in_state[0],
        Not(NotVariants::BufferGate)  =>  in_state[0],
        And(AndVariants::AndGate)     =>  in_state[0] &  in_state[1],
        And(AndVariants::NandGate)    => !(in_state[0] &  in_state[1]),
        Xor(XorVariants::XorGate)     =>  in_state[0] ^  in_state[1],
        Xor(XorVariants::XnorGate)    => !(in_state[0] ^  in_state[1]),
        Or (OrVariants ::OrGate)      =>  in_state[0] |  in_state[1],
        Or (OrVariants ::NorGate)     => !(in_state[0] |  in_state[1]),
        Latch(LatchVariants::DFlipFlop)=>{
            let d   = in_state[0];
            let clk = in_state[1];
            if clk { d } else { voxel.state }
        }
        _ => return None,
    };

    (ns != voxel.state).then_some(ns)
}

// 6-way neighbourhood reused everywhere.
const DIRS: [IVec3; 6] = [
    IVec3::new( 1, 0, 0), IVec3::new(-1, 0, 0),
    IVec3::new( 0, 1, 0), IVec3::new( 0,-1, 0),
    IVec3::new( 0, 0, 1), IVec3::new( 0, 0,-1),
];

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
