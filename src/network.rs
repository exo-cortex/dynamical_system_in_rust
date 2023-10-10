// use crate::{global_parameter_map::GlobalParameterMap, var::Var};

use rand::{rngs::SmallRng, Rng, SeedableRng};
use std::fmt;

// const DOMAIN: &'static str = "network";

#[allow(dead_code)]
pub enum SelectGroup {
    // #[default]
    AllGroups,
    SingleGroup(usize),
    NotGroup(usize),
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub struct Edge {
    pub group: usize, // assigning edges into groups in order to manipulate them by group
    pub into: usize,
    pub from: usize,
    pub strength: f64,
    pub turn: f64, // angle in `turns` [0, 1) instead of radians [0,2Pi)
    pub delay: f64,
}

impl Default for Edge {
    fn default() -> Self {
        Edge {
            group: 0,
            into: 0,
            from: 0,
            strength: 0.05,
            turn: 0.0,
            delay: 100.0,
        }
    }
}

#[allow(dead_code)]
pub struct Network {
    pub nodes: usize,
    pub edges: Vec<Edge>,
    pub edge_groups: Vec<String>,
    pub default_strength: f64,
    pub default_turn: f64,
    pub default_delay: f64,
    pub rng: SmallRng,
    pub dt: f64,
}

impl Default for Network {
    fn default() -> Self {
        Network {
            nodes: 1,
            edges: Vec::new(),
            edge_groups: Vec::new(),
            default_strength: 0.05,
            default_turn: 0.0,
            default_delay: 100.0,
            rng: SmallRng::seed_from_u64(0),
            dt: 1.0 / 64.0,
        }
    }
}

#[allow(dead_code)]
impl Network {
    // maybe implement default initialization again ...
    pub fn new(
        nodes: usize,
        default_strength: f64,
        default_turn: f64,
        default_delay: f64,
        seed: u64,
        dt: f64,
    ) -> Self {
        Network {
            nodes,
            edges: Vec::new(),
            edge_groups: Vec::new(),
            default_strength,
            default_turn,
            default_delay,
            //
            rng: SmallRng::seed_from_u64(seed),
            //
            dt,
        }
    }

    // pub fn place_parameters(parameters: &mut GlobalParameterMap) {
    //     parameters.insert(
    //         DOMAIN,
    //         "nodes",
    //         vec![Var::UInt(1)],
    //         vec!["N", "Nr", "nodes"],
    //         "amount of vertices in the network",
    //         true,
    //     );
    //     parameters.insert(
    //         DOMAIN,
    //         "default_strength",
    //         vec![Var::UFloat(0.1)],
    //         vec!["strength", "kappa", "k"],
    //         "default coupling strength of the network",
    //         true,
    //     );
    //     parameters.insert(
    //         DOMAIN,
    //         "default_turn",
    //         vec![Var::Turn(0.0)],
    //         vec!["turn", "phi"],
    //         "default coupling phase [turns] of the network",
    //         true,
    //     );
    //     parameters.insert(
    //         DOMAIN,
    //         "default_delay",
    //         vec![Var::UFloat(0.1)],
    //         vec!["delay", "tau"],
    //         "default coupling delay of the network",
    //         true,
    //     );
    //     // parameters that can occur multiple times
    //     parameters.insert(
    //         DOMAIN,
    //         "edge_placements",
    //         vec![
    //             Var::Int(0),
    //             Var::Int(0),
    //             Var::UFloat(0.1),
    //             Var::Turn(0.0),
    //             Var::UFloat(100.0),
    //         ],
    //         vec!["edge", "single-edge"],
    //         "command to place an edge into the network. usage: `[into from strength turn delay]` (can be set multiple times)",
    //         false,
    //     );
    //     parameters.insert(
    //         DOMAIN,
    //         "ring_clockwise",
    //         vec![
    //             Var::UFloat(0.1),
    //             Var::Turn(0.0),
    //             Var::UFloat(100.0),
    //         ],
    //         vec!["ring", "forward-ring", "ring-clockwise"],
    //         "command to place a ring into the network. usage: `[into from strength turn delay]` (can be set multiple times)",
    //         false,
    //     );

    //     parameters.insert(
    //         DOMAIN,
    //         "ring_counter_clockwise",
    //         vec![
    //             Var::UFloat(0.1),
    //             Var::Turn(0.0),
    //             Var::UFloat(100.0),
    //         ],
    //         vec!["ring-back", "backward-ring", "ring-counter-clockwise"],
    //         "command to place a backwards ring into the network. usage: `[into from strength turn delay]` (can be set multiple times)",
    //         false,
    //     );
    //     parameters.insert(
    //         DOMAIN,
    //         "ring_bidirectional",
    //         vec![
    //             Var::UFloat(0.1),
    //             Var::Turn(0.0),
    //             Var::UFloat(100.0),
    //         ],
    //         vec!["bidir-ring", "bi-ring", "ring-bidirectional"],
    //         "command to place a bidirectional ring: `[strength turn delay]` (can be set multiple times)",
    //         false,
    //     );

    //     parameters.insert(
    //         DOMAIN,
    //         "jumps",
    //         vec![
    //             Var::Int(0),
    //             Var::UInt(2),
    //             Var::Int(2),
    //             Var::UFloat(0.1),
    //             Var::Turn(0.0),
    //             Var::UFloat(100.0),
    //         ],
    //         vec!["jumps"],
    //         "command to place a chain of connected nodes: `[offset jump-separation jump-length strength turn delay]` (can be set multiple times)",
    //         false,
    //     );

    //     parameters.insert(
    //         DOMAIN,
    //         "chain",
    //         vec![
    //             Var::Int(0),
    //             Var::Int(1),
    //             Var::UFloat(0.1),
    //             Var::Turn(0.0),
    //             Var::UFloat(100.0),
    //         ],
    //         vec!["chain"],
    //         "command to place a chain of connected nodes: `[start end strength turn delay]` (can be set multiple times)",
    //         false,
    //     );

    //     parameters.insert(
    //         DOMAIN,
    //         "randomize_edge_strengths",
    //         vec![Var::UFloat(0.1)],
    //         vec!["random-strength"],
    //         "command that randomizes edge strengths",
    //         false,
    //     )
    // }

    // pub fn from_parameters(parameters: &GlobalParameterMap) -> Self {
    //     Network {
    //         nodes: parameters.get(DOMAIN, "nodes").parameter[0][0]
    //             .get::<u32>()
    //             .expect("no nodes parameter set") as usize,
    //         edges: Vec::new(),
    //         edge_groups: Vec::new(),
    //         default_strength: parameters.get(DOMAIN, "default_strength").parameter[0][0]
    //             .get::<f64>()
    //             .expect("no default strength parameter set"),
    //         default_turn: parameters.get(DOMAIN, "default_turn").parameter[0][0]
    //             .get::<f64>()
    //             .expect("no default coupling angle (in turns) set"),
    //         default_delay: parameters.get(DOMAIN, "default_delay").parameter[0][0]
    //             .get::<f64>()
    //             .expect("no default coupling delay set"),
    //         rng: SmallRng::seed_from_u64(
    //             parameters.get("general", "seed").parameter[0][0]
    //                 .get::<u32>()
    //                 .unwrap_or_default() as u64,
    //         ),
    //         dt: parameters.get("general", "dt").parameter[0][0]
    //             .get::<f64>()
    //             .expect("no integration stepsize set"),
    //     }
    // }

    // pub fn setup_connections(&mut self, parameters: &GlobalParameterMap) {
    //     let edge_parameter = &parameters.get(DOMAIN, "edge_placements");
    //     if edge_parameter.was_set {
    //         for parameter_set in &edge_parameter.parameter {
    //             let into = parameter_set[0].get::<i32>().unwrap() as isize;
    //             let from = parameter_set[1].get::<i32>().unwrap() as isize;
    //             let strength = parameter_set[2].get::<f64>().unwrap();
    //             let turn = parameter_set[3].get::<f64>().unwrap();
    //             let delay = parameter_set[4].get::<f64>().unwrap();
    //             self.put_edge(into, from, strength, turn, delay);
    //         }
    //     }

    //     let ring_forward_parameter = &parameters.get(DOMAIN, "ring_clockwise");
    //     if ring_forward_parameter.was_set {
    //         for parameter_set in &ring_forward_parameter.parameter {
    //             let strength = parameter_set[0].get::<f64>().unwrap();
    //             let turn = parameter_set[1].get::<f64>().unwrap();
    //             let delay = parameter_set[2].get::<f64>().unwrap();
    //             self.put_ring(strength, turn, delay);
    //         }
    //     }

    //     let ring_backward_parameter = &parameters.get(DOMAIN, "ring_counter_clockwise");
    //     if ring_backward_parameter.was_set {
    //         for parameter_set in &ring_backward_parameter.parameter {
    //             let strength = parameter_set[0].get::<f64>().unwrap();
    //             let turn = parameter_set[1].get::<f64>().unwrap();
    //             let delay = parameter_set[2].get::<f64>().unwrap();
    //             self.put_ring_reverse(strength, turn, delay);
    //         }
    //     }

    //     let ring_bidir_parameter = &parameters.get(DOMAIN, "ring_bidirectional");
    //     if ring_bidir_parameter.was_set {
    //         for parameter_set in &ring_bidir_parameter.parameter {
    //             let strength = parameter_set[0].get::<f64>().unwrap();
    //             let turn = parameter_set[1].get::<f64>().unwrap();
    //             let delay = parameter_set[2].get::<f64>().unwrap();
    //             self.put_bi_ring(strength, turn, delay);
    //         }
    //     }

    //     let chain_parameter = &parameters.get(DOMAIN, "chain");
    //     if chain_parameter.was_set {
    //         for parameter_set in &chain_parameter.parameter {
    //             let start = parameter_set[0].get::<i32>().unwrap() as isize;
    //             let end = parameter_set[1].get::<i32>().unwrap() as isize;
    //             let strength = parameter_set[2].get::<f64>().unwrap();
    //             let turn = parameter_set[3].get::<f64>().unwrap();
    //             let delay = parameter_set[4].get::<f64>().unwrap();
    //             self.put_chain(start, end, strength, turn, delay);
    //         }
    //     }

    //     let jumps_parameter = &parameters.get(DOMAIN, "jumps");
    //     if jumps_parameter.was_set {
    //         for parameter_set in &jumps_parameter.parameter {
    //             let offset = parameter_set[0].get::<i32>().unwrap() as isize;
    //             let separation = parameter_set[1].get::<u32>().unwrap() as usize;
    //             let length = parameter_set[2].get::<i32>().unwrap() as isize;
    //             let strength = parameter_set[3].get::<f64>().unwrap();
    //             let turn = parameter_set[4].get::<f64>().unwrap();
    //             let delay = parameter_set[5].get::<f64>().unwrap();
    //             self.put_jumps(offset, separation, length, strength, turn, delay);
    //         }
    //     }

    //     // network manipulation
    //     let strength_randomization = &parameters.get(DOMAIN, "randomize_edge_strengths");
    //     if strength_randomization.was_set {
    //         for parameter_set in &strength_randomization.parameter {
    //             let amount = parameter_set[0].get::<f64>().unwrap();
    //             self.randomize_strength(amount, SelectGroup::AllGroups);
    //         }
    //     }
    // }

    fn new_edge(
        group: usize,
        into: usize,
        from: usize,
        strength: f64,
        turn: f64,
        delay: f64,
    ) -> Edge {
        Edge {
            group,
            into,
            from,
            strength,
            turn,
            delay,
        }
    }

    // base edge insertion method
    fn insert_edge(&mut self, into: usize, from: usize, strength: f64, turn: f64, delay: f64) {
        self.edges.push(Self::new_edge(
            self.edge_groups.len(),
            into,
            from,
            strength,
            turn,
            delay,
        ));
    }

    // construct network
    pub fn put_edge(&mut self, into: isize, from: isize, strength: f64, turn: f64, delay: f64) {
        self.insert_edge(
            on_ring(into, self.nodes),
            on_ring(from, self.nodes),
            // (((into % self.nodes as isize) + self.nodes as isize) % self.nodes as isize) as usize,
            // (((from % self.nodes as isize) + self.nodes as isize) % self.nodes as isize) as usize,
            strength,
            turn,
            delay,
        );
        self.edge_groups.push("single edge".to_string());
    }

    pub fn put_diag(&mut self, offset: i16, strength: f64, turn: f64, delay: f64) {
        let actual_offset = ((offset % self.nodes as i16) + self.nodes as i16) % self.nodes as i16; // argh!!!
        for n in 0..self.nodes {
            self.insert_edge(n, (n + offset as usize) % self.nodes, strength, turn, delay);
        }
        let offset_string = format!("diagonal with offset {}", actual_offset % self.nodes as i16);
        self.edge_groups.push(offset_string);
    }
    pub fn put_ring(&mut self, strength: f64, turn: f64, delay: f64) {
        for n in 0..self.nodes {
            self.insert_edge((n + 1) % self.nodes, n, strength, turn, delay);
        }
        self.edge_groups.push("ring --> (clockwise)".to_string());
    }
    pub fn put_ring_reverse(&mut self, strength: f64, turn: f64, delay: f64) {
        for n in 0..self.nodes {
            self.insert_edge(n, (n + 1) % self.nodes, strength, turn, delay)
        }
        self.edge_groups
            .push("ring <-- (counter-clockwise)".to_string());
    }
    pub fn put_bi_ring(&mut self, strength: f64, turn: f64, delay: f64) {
        for n in 0..self.nodes {
            self.insert_edge(
                on_ring(n as isize, self.nodes),
                on_ring(n as isize + 1, self.nodes),
                strength,
                turn,
                delay,
            );
        }
        for n in 0..self.nodes {
            self.insert_edge(
                on_ring(n as isize, self.nodes),
                on_ring(n as isize - 1, self.nodes),
                strength,
                turn,
                delay,
            );
        }
        self.edge_groups.push("bidirectional ring".to_string());
    }
    pub fn put_jumps(
        // are there better names for "jump length" and "jump separation"?
        // works like this
        // (0)   (1)   (2)   (3)   (4)   (5)   (6)   (7) ...
        // offs-->|                 |           |
        //        +-----+(<- jump length)-+     +-----+
        //              |                 |           |
        // (0)   (1)   (2)   (3)   (4)   (5)   (6)   (7)
        //              +<jump_separation>+
        &mut self,
        offset: isize,
        jump_separation: usize,
        jump_length: isize,
        strength: f64,
        turn: f64,
        delay: f64,
    ) {
        for n in 0..self.nodes / jump_separation {
            self.insert_edge(
                on_ring(offset + (n * jump_separation) as isize, self.nodes),
                on_ring(
                    offset + (n * jump_separation) as isize + jump_length,
                    self.nodes,
                ),
                strength,
                turn,
                delay,
            );
        }
        let description = format!(
            "jumps (separation: {}, distance: {})",
            jump_separation, jump_length
        );
        self.edge_groups.push(description)
    }

    pub fn put_chain(&mut self, start: isize, end: isize, strength: f64, turn: f64, delay: f64) {
        if start < end {
            for i in start..end {
                println!(
                    " {} <-- {}",
                    on_ring(i + 1, self.nodes),
                    on_ring(i, self.nodes)
                );
                self.insert_edge(
                    on_ring(i + 1, self.nodes),
                    on_ring(i, self.nodes),
                    strength,
                    turn,
                    delay,
                );
            }
        }
        let edge_group_name = format!("chain [{start}..{end}]").to_string();
        self.edge_groups.push(edge_group_name);
    }

    // modify network
    pub fn turn_angles(&mut self, turn: f64, selection: SelectGroup) {
        match selection {
            SelectGroup::SingleGroup(which_group) => {
                if which_group < self.edge_groups.len() {
                    for edge in &mut self.edges {
                        if edge.group == which_group {
                            edge.turn = ((edge.turn + turn) % 1.0 + 1.0) % 1.0
                        }
                    }
                }
            }
            SelectGroup::NotGroup(which_group) => {
                if which_group < self.edge_groups.len() {
                    for edge in &mut self.edges {
                        if edge.group != which_group {
                            edge.turn = ((edge.turn + turn) % 1.0 + 1.0) % 1.0
                        }
                    }
                }
            }
            SelectGroup::AllGroups => {
                for edge in &mut self.edges {
                    edge.turn = ((edge.turn + turn) % 1.0 + 1.0) % 1.0
                }
            }
        }
    }

    pub fn randomize_strength(&mut self, amount: f64, selection: SelectGroup) {
        match selection {
            SelectGroup::SingleGroup(which) => {
                if which < self.edges.len() {
                    for edge in &mut self.edges {
                        if edge.group == which {
                            edge.strength += self.rng.gen_range(-1.0..1.0) * amount;
                        }
                    }
                }
            }
            SelectGroup::NotGroup(which) => {
                if which < self.edges.len() {
                    for edge in &mut self.edges {
                        edge.strength += self.rng.gen_range(-1.0..1.0) * amount;
                    }
                }
            }
            SelectGroup::AllGroups => {
                for edge in &mut self.edges {
                    edge.strength += self.rng.gen_range(-1.0..1.0) * amount;
                }
            }
        }
    }

    pub fn randomize_angle(&mut self, amount: f64, selection: SelectGroup) {
        match selection {
            SelectGroup::SingleGroup(which) => {
                if which < self.edges.len() {
                    for edge in &mut self.edges {
                        if edge.group == which {
                            edge.turn =
                                ((edge.turn + self.rng.gen_range(-1.0..1.0) * amount) % 1.0 + 1.0)
                                    % 1.0;
                        }
                    }
                }
            }
            SelectGroup::NotGroup(which) => {
                if which < self.edges.len() {
                    for edge in &mut self.edges {
                        edge.turn = ((edge.turn + self.rng.gen_range(-1.0..1.0) * amount) % 1.0
                            + 1.0)
                            % 1.0;
                    }
                }
            }
            SelectGroup::AllGroups => {
                for edge in &mut self.edges {
                    edge.turn =
                        ((edge.turn + self.rng.gen_range(-1.0..1.0) * amount) % 1.0 + 1.0) % 1.0;
                }
            }
        }
    }

    pub fn randomize_delay_relative(&mut self, rel_amount: f64, selection: SelectGroup) {
        if rel_amount < 1.0 {
            match selection {
                SelectGroup::SingleGroup(which) => {
                    if which < self.edges.len() {
                        for edge in &mut self.edges {
                            if edge.group == which {
                                edge.delay *= 1.0 + self.rng.gen_range(-1.0..1.0) * rel_amount;
                            }
                        }
                    }
                }
                SelectGroup::NotGroup(which) => {
                    if which < self.edges.len() {
                        for edge in &mut self.edges {
                            edge.delay *= 1.0 + self.rng.gen_range(-1.0..1.0) * rel_amount;
                        }
                    }
                }
                SelectGroup::AllGroups => {
                    for edge in &mut self.edges {
                        edge.delay *= 1.0 + self.rng.gen_range(-1.0..1.0) * rel_amount;
                    }
                }
            }
        } else {
            panic!("amount of relative randomness cannot be larger than 1.0.")
        }
    }

    // convert into different formats ?
    // maintainance
    // if two edges share the same values for {from, into, delay} complex coupling strengths can be added.
    pub fn simplify_network(&mut self) {
        self.combine_edges(); // two (or more) edges with the same values for (from, into, delay) can be combined into one.
        self.remove_irrelevant_edges(); // edges with strength = 0 shouldn't be computed
    }
    fn combine_edges(&mut self) {
        println!("\"combine_edges()\" - not yet implemented.");
        // if (into, from, delay) are the same for different edges the (complex) coupling strengths can be summed and only one edge has to remain
        // maybe only use to convert edges from polar coordinates (strength, angle) to complex values `kappa=(re, im)`
    }
    fn remove_irrelevant_edges(&mut self) {
        // make me private use as in "simplify network"
        self.edges.retain(|&e| e.strength > 0.0);
    }

    pub fn get_nodes(&self) -> usize {
        self.nodes
    }

    pub fn get_edge_groups(&self) -> usize {
        self.edge_groups.len()
    }

    pub fn get_edges_into_node(&self, into: usize) -> Vec<Edge> {
        self.edges
            .iter()
            .filter_map(|e| {
                if e.into == into {
                    Some(e.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn get_edges_into_nodes(&self) -> Vec<Vec<Edge>> {
        (0..self.nodes)
            .into_iter()
            .map(|into| self.get_edges_into_node(into).clone())
            .collect()
    }
}

impl fmt::Display for Network {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "Network:\n{} nodes, {} edges in {} edge-groups.",
            self.nodes,
            self.edges.len(),
            self.edge_groups.len()
        )
        .unwrap();
        for (g, groupname) in self.edge_groups.iter().enumerate() {
            writeln!(f, "edge-group [{}] \"{}\":", g, groupname).unwrap();
            for n in 0..self.nodes {
                for edge in &self.edges {
                    if edge.group == g && edge.into == n {
                        writeln!(
							f,
							"\t{:2} <<< {:2}: kappa = {:.3}, angle = {:.3} * 2Pi, tau = {:.2} (steps: {})",
							n, edge.from, edge.strength, edge.turn, edge.delay, (edge.delay / self.dt) as usize
						)
                        .unwrap();
                    }
                }
            }
        }
        Ok(())
    }
}

fn on_ring(a: isize, b: usize) -> usize {
    (((a % b as isize) + b as isize) % b as isize) as usize
}
