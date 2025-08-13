use crate::prelude::*; 

/// Returns `true` when `voxel` is capable of transporting **`channel`**.
#[inline(always)]
fn carries(voxel: &Voxel, channel: u8) -> bool {
    match voxel.kind {
        VoxelType::Wire(ch)    => ch == channel,
        VoxelType::BundledWire => true,
        _                      => false,
    }
}

/// Ensures `word` only stores information that *`kind`* is allowed to keep.
///
/// * A *bundled* wire (or any gate) can keep the full 16‑bit word unchanged.
/// * A single‑channel `Wire(n)` must clamp the word to **exactly** the state
///   of bit‑`n` and clear everything else.
#[inline]
fn clamp_state(kind: &VoxelType, mut word: Bits16) -> Bits16 {
    if let VoxelType::Wire(ch) = *kind {
        let keep = word.get(ch);
        word = Bits16::all_zeros();
        if keep { word.set(ch); }
    }
    word
}


#[derive(Resource)]
pub struct SimulationTimer {
    pub tick: Timer,
    pub rate: u64, 
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

// ────────────────────────────────────────────────────────────────────────────
// 2.  Logic‑system without any dirty‑tracking
// ────────────────────────────────────────────────────────────────────────────
pub fn logic_system(
    time: Res<Time>,
    mut sim_timer: ResMut<SimulationTimer>,
    voxel_map: ResMut<VoxelMap>,
    mut logic_writer: EventWriter<LogicEvent>,
) { 
    // advance the clock
    sim_timer.tick.tick(time.delta());
    if !sim_timer.tick.finished() { return; }

    // ── A. Re‑simulate *every* gate ─────────────────────────────────────────
    for (&pos, voxel) in voxel_map.voxel_map.iter() {
        if let Some(new_state) = simulate_gate(voxel, &voxel_map) {
            logic_writer.send(LogicEvent::UpdateVoxel {
                position: pos,
                new_state,
            });

            // if the gate drives a plain wire, update that wire immediately
            let (_, output) = voxel_directions(voxel);
            if let Some(out_voxel) = voxel_map.voxel_map.get(&output) {
                if matches!(out_voxel.kind, VoxelType::Wire(_)) {
                    logic_writer.send(LogicEvent::UpdateVoxel {
                        position: output,
                        new_state,
                    });
                }
            }
        }
    }

    // ── B. Re‑propagate *all* wires ─────────────────────────────────────────
    for event in propagate_wires(&voxel_map) {
        logic_writer.send(event);
    }
}



pub fn propagate_wires(voxel_map: &VoxelMap) -> Vec<LogicEvent> {
    use std::collections::{HashMap, HashSet, VecDeque};

    // --- 1. gather every *gate* output word ---------------------------------
    let mut gate_drive: HashMap<IVec3, Bits16> = HashMap::new();

    for (_, v) in &voxel_map.voxel_map {
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
                            if driven_high { w.set(ch) } else { w.clear(ch) }
                            *w = clamp_state(&voxel_map.voxel_map[&p].kind, *w);
                        })
                        
                        // inside or_insert_with
                        .or_insert_with(|| {
                            let mut w = cur_word;
                            if driven_high { w.set(ch) } else { w.clear(ch) }
                            clamp_state(&voxel_map.voxel_map[&p].kind, w)
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
            let d   = in_sig[1];
            let clk = in_sig[0];
            if clk { d } else { voxel.state.any_set() }
        }
        
        Component(ComponentVariants::Light) => in_sig[0],
        
        _ => return None,   // voxels that aren’t logic gates
    };

    let new_state = bitword(out_bool);

    (new_state != voxel.state).then_some(new_state)
}

pub fn voxel_directions(voxel: &Voxel) -> (Vec<IVec3>, IVec3) {
    // local → world rotation (XZ only)
    let rot = |v: IVec3| match voxel.direction {
        1 => v,                         // +Z forward
        2 => IVec3::new( v.z, 0, -v.x), // +X forward:  (x,z) ↦ ( z,-x)
        3 => IVec3::new(-v.x, 0, -v.z), // –Z forward
        4 => IVec3::new(-v.z, 0,  v.x), // –X forward
        d => { eprintln!("bad dir {d}"); v }
    };

    let IoPattern { inputs, output } = voxel.kind.io_pattern();

    // rotate + translate
    let world_inputs : Vec<IVec3> = inputs.iter().map(|&v| voxel.position + rot(v)).collect();
    let world_output              = output.map_or(IVec3::ZERO, |v| voxel.position + rot(v));
    
    (world_inputs, world_output)
}
