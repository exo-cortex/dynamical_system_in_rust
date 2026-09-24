use crate::{SelectGroup, edge::Edge, on_ring};
use rand::Rng;
use std::fmt::{Display, Formatter, Result};

#[allow(dead_code)]
pub struct Network {
    pub nodes: usize,
    pub edges: Vec<Edge>,
    pub edge_groups: Vec<String>,
    pub default_strength: f64,
    pub default_turn: f64,
    pub default_delay: f64,
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
        }
    }
}

#[allow(dead_code)]
impl Network {
    // maybe implement default initialization again ...
    pub fn new(nodes: usize, default_strength: f64, default_turn: f64, default_delay: f64) -> Self {
        Network {
            nodes,
            edges: Vec::new(),
            edge_groups: Vec::new(),
            default_strength,
            default_turn,
            default_delay,
        }
    }

    // base edge insertion method
    fn create_edge(&mut self, into: usize, from: usize, strength: f64, turn: f64, delay: f64) {
        let new_edge = Edge {
            group: self.edge_groups.len(),
            into,
            from,
            strength,
            turn,
            delay,
        };

        self.edges.push(new_edge);
    }

    // construct network
    pub fn put_edge(&mut self, into: isize, from: isize, strength: f64, turn: f64, delay: f64) {
        self.create_edge(
            on_ring(into, self.nodes),
            on_ring(from, self.nodes),
            strength,
            turn,
            delay,
        );
        self.edge_groups.push("single edge".to_string());
    }

    pub fn put_diag(&mut self, offset: i16, strength: f64, turn: f64, delay: f64) {
        let actual_offset = ((offset % self.nodes as i16) + self.nodes as i16) % self.nodes as i16; // argh!!!
        for n in 0..self.nodes {
            self.create_edge(n, (n + offset as usize) % self.nodes, strength, turn, delay);
        }
        let offset_string = format!("diagonal with offset {}", actual_offset % self.nodes as i16);
        self.edge_groups.push(offset_string);
    }

    pub fn put_ring(&mut self, strength: f64, turn: f64, delay: f64) {
        for n in 0..self.nodes {
            self.create_edge((n + 1) % self.nodes, n, strength, turn, delay);
        }
        self.edge_groups.push("ring --> (clockwise)".to_string());
    }

    pub fn put_ring_reverse(&mut self, strength: f64, turn: f64, delay: f64) {
        for n in 0..self.nodes {
            self.create_edge(n, (n + 1) % self.nodes, strength, turn, delay)
        }
        self.edge_groups
            .push("ring <-- (counter-clockwise)".to_string());
    }

    pub fn put_bi_ring(&mut self, strength: f64, turn: f64, delay: f64) {
        for n in 0..self.nodes {
            self.create_edge(
                on_ring(n as isize, self.nodes),
                on_ring(n as isize + 1, self.nodes),
                strength,
                turn,
                delay,
            );
        }

        for n in 0..self.nodes {
            self.create_edge(
                on_ring(n as isize, self.nodes),
                on_ring(n as isize - 1, self.nodes),
                strength,
                turn,
                delay,
            );
        }
        self.edge_groups.push("bidirectional ring <->".to_string());
    }
    /// are there better names for "jump length" and "jump separation"?
    /// example with 8 nodes:
    /// works like this
    /// to-nodes:   (0)   (1)   (2)   (3)   (4)   (5)   (6)   (7)   (8) ...
    ///          offsets-->|                 |           |
    ///                    +-----+(<- jump length)-+     +-----+
    ///                          |                 |           |
    /// from-nodes: (0)   (1)   (2)   (3)   (4)   (5)   (6)   (7)   (8)
    ///                          +<jump_separation>+
    pub fn put_jumps(
        &mut self,
        offset: isize,
        jump_separation: usize,
        jump_length: isize,
        strength: f64,
        turn: f64,
        delay: f64,
    ) {
        for n in 0..self.nodes / jump_separation {
            self.create_edge(
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
                self.create_edge(
                    on_ring(i + 1, self.nodes),
                    on_ring(i, self.nodes),
                    strength,
                    turn,
                    delay,
                );
            }
        }
        let edge_group_name = format!("chain [{start}..{end}]");
        self.edge_groups.push(edge_group_name);
    }

    pub fn get_edges(&mut self, selection: SelectGroup) -> Vec<&mut Edge> {
        match selection {
            SelectGroup::SingleGroup(which_group) if which_group < self.edge_groups.len() => self
                .edges
                .iter_mut()
                .filter(|e| e.group == which_group)
                .collect(),
            SelectGroup::NotGroup(which_group) if which_group < self.edge_groups.len() => self
                .edges
                .iter_mut()
                .filter(|e| e.group != which_group)
                .collect(),
            SelectGroup::AllGroups => self.edges.iter_mut().map(|e| e).collect(),
            _ => Vec::new(),
        }
    }

    pub fn turn_angles(&mut self, turn: f64, selection: SelectGroup) {
        for edge in self.get_edges(selection) {
            edge.turn = (edge.turn + turn).rem_euclid(1.0); // calculates the least nonnegative remainder
        }
    }

    pub fn randomize_strength(&mut self, amount: f64, selection: SelectGroup, rng: &mut impl Rng) {
        for edge in self.get_edges(selection) {
            edge.strength += rng.gen_range(-1.0..1.0) * amount;
            edge.strength = edge.strength.abs();
        }
    }

    pub fn randomize_angle(&mut self, amount: f64, selection: SelectGroup, rng: &mut impl Rng) {
        for edge in self.get_edges(selection) {
            edge.turn = ((edge.turn + rng.gen_range(-1.0..1.0) * amount) % 1.0 + 1.0) % 1.0;
        }
    }

    pub fn randomize_delay_relative(
        &mut self,
        rel_amount: f64,
        selection: SelectGroup,
        rng: &mut impl Rng,
    ) {
        if rel_amount < 1.0 {
            for edge in self.get_edges(selection) {
                edge.delay *= 1.0 + rng.gen_range(-1.0..1.0) * rel_amount;
            }
        } else {
            panic!("amount of relative randomness cannot be larger than 1.0.")
        }
    }

    pub fn remove_zero_strength_edges(&mut self) {
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
            .filter_map(|e| if e.into == into { Some(*e) } else { None })
            .collect()
    }

    pub fn get_edges_into_nodes(&self) -> Vec<Vec<Edge>> {
        (0..self.nodes)
            .map(|into| self.get_edges_into_node(into).clone())
            .collect()
    }
}

impl Display for Network {
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(
            f,
            "Network:\n{} nodes, {} edges in {} edge-groups.",
            self.nodes,
            self.edges.len(),
            self.edge_groups.len()
        )?;
        for (g, groupname) in self.edge_groups.iter().enumerate() {
            writeln!(f, "edge-group [{}] \"{}\":", g, groupname).unwrap();
            for n in 0..self.nodes {
                for edge in &self.edges {
                    if edge.group == g && edge.into == n {
                        writeln!(
							f,
							"\t{:2} <<< {:2}: kappa = {:.3}, angle = {:.3} * 2Pi, tau = {:.2} (steps: {})",
							n, edge.from, edge.strength, edge.turn, edge.delay, edge.delay
						)
                        .unwrap();
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_selection() {
        let mut network: Network = Network::new(4, 0.05, 0.0, 100.0);

        assert_eq!(1, 1);
    }
}
