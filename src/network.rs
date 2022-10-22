use crate::history::ReadAtMultiply;
use num_complex::Complex;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use std::f64::consts::PI;
use std::fmt;

#[allow(dead_code)]
// #[derive(Default)] // doesn't work on desktop but on laptop ? nightly issue?
pub enum SelectGroup {
    // #[default]
    AllGroups,
    SingleGroup(usize),
    NotGroup(usize),
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub struct Edge {
    group: usize, // assigning edges into groups in order to manipulate them by group
    into: usize,
    from: usize,
    strength: f64,
    turn: f64, // angle in `turns` [0, 1) instead of radians [0,2Pi)
    delay: f64,
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
    pub default_strength: Option<f64>,
    pub default_turn: Option<f64>,
    pub default_delay: Option<f64>,
    pub rng: SmallRng,
    pub dt: f64,
}

impl Default for Network {
    fn default() -> Self {
        Network {
            nodes: 1,
            edges: Vec::new(),
            edge_groups: Vec::new(),
            default_strength: Some(0.05),
            default_turn: Some(0.0),
            default_delay: Some(100.0),
            rng: SmallRng::seed_from_u64(0),
            dt: 1.0 / 64.0,
        }
    }
}

// ideally use with fluent interface ... but how?
// put_edge(...).strength(...).turn(...).delay(...);
// put_ring(...).strength(...).turn(...).delay(...);

#[allow(dead_code)]
impl Network {
    // maybe implement default initialization again ...
    pub fn new(
        nodes: usize,
        default_strength: Option<f64>,
        default_turn: Option<f64>,
        default_delay: Option<f64>,
        seed: Option<u64>,
        dt: f64,
    ) -> Self {
        Network {
            nodes,
            edges: Vec::new(),
            edge_groups: Vec::new(),
            default_strength,
            default_turn,
            default_delay,
            rng: SmallRng::seed_from_u64(seed.unwrap_or_default()),
            dt,
        }
    }

    // construct network
    pub fn put_edge(&mut self, into: usize, from: usize, strength: f64, turn: f64, delay: f64) {
        self.edges.push(Edge {
            group: self.edge_groups.len(),
            into,
            from,
            strength,
            turn,
            delay,
        });
        self.edge_groups.push("single edge".to_string());
    }
    pub fn put_diag(&mut self, offset: i16, strength: f64, turn: f64, delay: f64) {
        let actual_offset = ((offset % self.nodes as i16) + self.nodes as i16) % self.nodes as i16; // argh!!!
        println!("offset {}", offset);
        for n in 0..self.nodes {
            self.edges.push(Edge {
                group: self.edge_groups.len(),
                into: n,
                // from: (n + actual_offset as usize) % self.nodes,
                from: (n + offset as usize) % self.nodes,
                strength,
                turn,
                delay,
            });
        }
        let offset_string = format!("diagonal with offset {}", actual_offset % self.nodes as i16);
        self.edge_groups.push(offset_string);
    }
    pub fn put_ring(&mut self, strength: f64, turn: f64, delay: f64) {
        for n in 0..self.nodes {
            self.edges.push(Edge {
                group: self.edge_groups.len(),
                into: n,
                from: (n + 1) % self.nodes,
                strength,
                turn,
                delay,
            });
        }
        self.edge_groups.push("ring <--".to_string());
    }
    pub fn put_ring_reverse(&mut self, strength: f64, turn: f64, delay: f64) {
        for n in 0..self.nodes {
            self.edges.push(Edge {
                group: self.edge_groups.len(),
                into: n,
                from: (n - 1) % self.nodes,
                strength,
                turn,
                delay,
            });
        }
        self.edge_groups.push("ring -->".to_string());
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
        offset: usize,
        jump_separation: usize, // better name?
        jump_distance: usize,   // better name
        strength: f64,
        turn: f64,
        delay: f64,
    ) {
        // if self.nodes / number != integer {warning!}
        for n in 0..self.nodes / jump_separation {
            self.edges.push(Edge {
                group: self.edge_groups.len(),
                into: (offset + n * jump_separation) % self.nodes,
                from: (offset + n * jump_separation + jump_distance) % self.nodes,
                strength,
                turn,
                delay,
            });
        }
        let description = format!(
            "jumps (separation: {}, distance: {})",
            jump_separation, jump_distance
        );
        self.edge_groups.push(description)
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
        // if (into, from, delay) are the same for different edges the (complex) coupling strengths can be summed and only one edge has to remain
        // maybe only use to convert edges from polar coordinates (strength, angle) to complex values `kappa=(re, im)`
    }
    fn remove_irrelevant_edges(&mut self) {
        // make me private use as in "simplify network"
        self.edges.retain(|&e| e.strength > 0.0);
    }

    pub fn get_edges_into_node(&self, into: usize) -> Vec<ReadAtMultiply<Complex<f64>>> {
        self.edges
            .iter()
            .filter(|e| e.into == into)
            .map(|e| ReadAtMultiply::<Complex<f64>> {
                at_node: into,
                at_delay: (e.delay / self.dt) as usize,
                weight: e.strength * (Complex::<f64>::new(0.0, 2.0 * PI)).exp(),
            })
            .collect()
    }

    pub fn get_edges_into_nodes(&self) -> Vec<Vec<ReadAtMultiply<Complex<f64>>>> {
        (0..self.nodes)
            .into_iter()
            .map(|into| {
                self.edges
                    .iter()
                    .filter(|e| e.into == into)
                    .map(|e| ReadAtMultiply::<Complex<f64>> {
                        at_node: e.from,
                        at_delay: (e.delay / self.dt) as usize,
                        weight: e.strength * (Complex::<f64>::new(0.0, 2.0 * PI)).exp(),
                    })
                    .collect()
            })
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
                            "\t{:2} <- {:2}: kappa = {:.2}, angle = {:.2} * 2Pi, tau = {:.1} (steps: {})",
                            n, edge.from, edge.strength, edge.turn, edge.delay, (edge.delay / self.dt) as usize
                        ).unwrap();
                    }
                }
            }
        }
        Ok(())
    }
}
