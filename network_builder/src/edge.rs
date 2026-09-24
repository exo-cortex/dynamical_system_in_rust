use rand::Rng;

const DEFAULT_STRENGTH: f64 = 0.05;
const DEFAULT_TURN: f64 = 0.0;
const DEFAULT_DELAY: f64 = 100.0;

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
            strength: DEFAULT_STRENGTH,
            turn: DEFAULT_TURN,
            delay: DEFAULT_DELAY,
        }
    }
}

pub struct EdgeBuilder {
    group: Option<usize>,
    into: Option<usize>,
    from: Option<usize>,
    strength: Option<f64>,
    turn: Option<f64>,
    delay: Option<f64>,
}

impl EdgeBuilder {
    pub fn new() -> Self {
        EdgeBuilder {
            group: None,
            into: None,
            from: None,
            strength: None,
            turn: None,
            delay: None,
        }
    }

    pub fn into(mut self, into: usize) -> Self {
        self.into = Some(into);
        self
    }

    pub fn from(mut self, from: usize) -> Self {
        self.from = Some(from);
        self
    }

    pub fn with_group_index(mut self, group: usize) -> Self {
        self.group = Some(group);
        self
    }

    pub fn build(self) -> Edge {
        Edge {
            group: self.group.unwrap_or(0),
            into: self.into.unwrap_or(0),
            from: self.from.unwrap_or(0),
            strength: self.strength.unwrap_or(DEFAULT_STRENGTH),
            turn: self.turn.unwrap_or(DEFAULT_TURN),
            delay: self.delay.unwrap_or(DEFAULT_DELAY),
        }
    }
}
