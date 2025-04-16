use std::collections::{HashSet, VecDeque};

use bevy::reflect::{Map, Set};

use crate::prelude::*; 

#[derive(Resource)]
pub struct SimulationResource {
    pub tick_timer: Timer,
    pub dirty_voxels: HashSet<IVec3>,
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
                            println!("Changed voxel at {:?}: {:?}", position, voxel);
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
                        match output_voxel.t {
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
        match voxel.t {
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
        if !matches!(voxel.t, VoxelType::Wire(_)) {
            continue;
        }

        // Record the type of wire for this connected component.
        let wire_type = voxel.t;

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
                    if neighbor_voxel.t == wire_type {
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

fn simulate_gate(voxel: &Voxel, voxel_map: &VoxelMap) -> Option<bool> {
    let (inputs, _) = voxel_directions(voxel);
    // Collect input states, defaulting to false if an input voxel isn't present.
    let input_states: Vec<bool> = inputs
        .iter()
        .map(|input| voxel_map.voxel_map.get(input).map_or(false, |v| v.state))
        .collect();

    // Determine the new state based on the voxel's gate type.
    let new_state = match voxel.t {
        VoxelType::Not(NotVariants::NotGate) => !input_states.get(0).copied().unwrap_or(false), // NOT gate
        VoxelType::Not(NotVariants::BufferGate) => input_states.get(0).copied().unwrap_or(false),    // BUFFER
        VoxelType::And(AndVariants::AndGate) => input_states.get(0).copied().unwrap_or(false) 
                  && input_states.get(1).copied().unwrap_or(false), // AND gate
        VoxelType::And(AndVariants::NandGate) => !(input_states.get(0).copied().unwrap_or(false) 
                  && input_states.get(1).copied().unwrap_or(false)),// NAND gate
        VoxelType::Xor(XorVariants::XorGate) => input_states.get(0).copied().unwrap_or(false)
                  ^ input_states.get(1).copied().unwrap_or(false),  // XOR gate
        _ => return None, // Unsupported voxel type; no simulation performed.
    };

    // Only return a new state if it differs from the current state.
    if voxel.state != new_state {
        Some(new_state)
    } else {
        None
    }
}



pub fn voxel_directions(voxel: &Voxel) -> (Vec<IVec3>, IVec3) {
    let mut inputs = Vec::new();
    // Use IVec3::ZERO if available; otherwise, use IVec3::new(0, 0, 0)
    let mut output = IVec3::new(0, 0, 0);
    let position = voxel.position;
    let is_single_input = 
        voxel.t == VoxelType::Not(NotVariants::BufferGate) 
        || voxel.t == VoxelType::Not(NotVariants::NotGate);

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
