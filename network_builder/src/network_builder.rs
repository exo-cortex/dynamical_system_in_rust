use crate::{
    edge::{Edge, EdgeBuilder},
    network::Network,
    on_ring,
};

/// Usage:
/// ```
/// let net = NetworkBuilder::new(4)
///     .with_defaults(0.05, 0.0, 100.0)
///     .default_ring_network()
///     .build();
/// ```
pub struct NetworkBuilder {
    nodes: usize,
    default_strength: Option<f64>,
    default_turn: Option<f64>,
    default_delay: Option<f64>,
    edges: Vec<Edge>,
    edge_groups: Vec<String>,
    groups: usize,
}

impl NetworkBuilder {
    pub fn new(nodes: usize) -> Self {
        NetworkBuilder {
            nodes,
            default_strength: None,
            default_turn: None,
            default_delay: None,
            edges: Vec::new(),
            edge_groups: Vec::new(),
            groups: 0,
        }
    }

    pub fn with_defaults(
        mut self,
        default_strength: f64,
        default_turn: f64,
        default_delay: f64,
    ) -> Self {
        self.default_strength = Some(default_strength);
        self.default_turn = Some(default_turn);
        self.default_delay = Some(default_delay);
        self
    }

    pub fn default_ring_network(mut self) -> Self {
        for node in 0..self.nodes {
            let new_edge = EdgeBuilder::new()
                .into(node)
                .from(on_ring((node + 1) as isize, self.nodes))
                .with_group_index(self.groups)
                .build();

            self.edges.push(new_edge);
        }
        self.edge_groups.push("ring --> (clockwise)".to_string());
        self.groups += 1;
        self
    }

    pub fn build(self) -> Network {
        let default_strength = self.default_strength.unwrap_or(0.05);
        let default_turn = self.default_turn.unwrap_or(0.0);
        let default_delay = self.default_delay.unwrap_or(100.0);

        let mut network = Network::new(self.nodes, default_strength, default_turn, default_delay);
        network.edges = self.edges;
        network.edge_groups = self.edge_groups;
        network
    }
}
