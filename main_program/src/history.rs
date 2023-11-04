use crate::dynamical_system::{Feedback, WeightFromEdge};
use crate::network::{Edge, Network};
use ringbuffer::{AllocRingBuffer, RingBuffer};
use std::fmt;
use std::mem;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct ReadAtMultiply<WeightT>
where
    WeightT: Sized + Clone,
{
    pub at_node: usize,
    pub at_delay: usize,
    pub weight: WeightT,
}

#[allow(dead_code)]
impl<WeightT> ReadAtMultiply<WeightT>
where
    WeightT: Sized + Clone,
{
    pub fn new(at_node: usize, at_delay: usize, weight: WeightT) -> Self {
        ReadAtMultiply {
            at_node,
            at_delay,
            weight,
        }
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct History<S, T>
// FeedbackT - delay type,
// WeightT float type for multiplication of delay in weighted sum.
where
    S: Feedback,
    T: Sized
        + Clone
        + Copy
        + Default
        + core::iter::Sum
        + std::ops::Add<Output = T>
        + std::ops::AddAssign
        + std::ops::Mul<S::WeightT, Output = T>,
{
    history: Vec<AllocRingBuffer<T>>,
    readers: Vec<Vec<ReadAtMultiply<S::WeightT>>>,
    dt: f64,
}

#[allow(dead_code)]
impl<S, T> History<S, T>
where
    S: Feedback,
    T: Sized
        + Clone
        + Copy
        + Default
        + core::iter::Sum
        + std::ops::Add<Output = T>
        + std::ops::AddAssign
        + std::ops::Mul<S::WeightT, Output = T>,
{
    pub fn new(dt: f64, network: &Network, equal_ringbuffers: bool) -> Self {
        let mut history = History {
            history: Vec::new(),
            readers: Vec::new(),
            dt,
        };
        history.setup_connections(&network, equal_ringbuffers);
        history
    }

    pub fn setup_connections(&mut self, network: &Network, equal_ringbuffers: bool) {
        // ++++
        // Self::edges_valid(network.nodes, &network.get_edges_into_nodes());

        // ++++
        let delay_steps_needed = Self::max_delay_steps_needed(
            network.nodes,
            &network.get_edges_into_nodes(),
            equal_ringbuffers,
        );
        let ringbuffers: Vec<AllocRingBuffer<T>> = delay_steps_needed
            .iter()
            .map(|delay| {
                let delay_steps = ((delay / self.dt) as usize).next_power_of_two();
                AllocRingBuffer::<T>::new(delay_steps)
            })
            .collect();

        self.history = ringbuffers;

        Self::initialize_history(None, &mut self.history);

        self.readers = network
            .get_edges_into_nodes()
            .iter()
            .map(|es| {
                es.iter()
                    .map(|e| ReadAtMultiply::<S::WeightT> {
                        at_node: e.from,
                        at_delay: (e.delay / self.dt) as usize,
                        weight: S::WeightT::from_edge(e),
                    })
                    .collect()
            })
            .collect();
    }

    pub fn get_node_feedback(&mut self, into: usize) -> T {
        self.readers[into]
            .iter()
            .map(|r| *self.history[r.at_node].get(-(r.at_delay as isize)).unwrap() * r.weight)
            .sum()
    }

    pub fn get_single_node_feedback_rk4(&mut self) -> [T; 2] {
        [
            self.readers[0]
                .iter()
                .map(|r| *self.history[r.at_node].get(-(r.at_delay as isize)).unwrap() * r.weight)
                .sum(),
            self.readers[0]
                .iter()
                .map(|r| {
                    *self.history[r.at_node]
                        .get(1 - (r.at_delay as isize))
                        .unwrap()
                        * r.weight
                })
                .sum(),
        ]
    }

    pub fn get_node_feedback_rk4(&mut self, into: usize) -> [T; 2] {
        [
            self.readers[into]
                .iter()
                .map(|r| *self.history[r.at_node].get(-(r.at_delay as isize)).unwrap() * r.weight)
                .sum(),
            self.readers[into]
                .iter()
                .map(|r| {
                    *self.history[r.at_node]
                        .get(1 - (r.at_delay as isize))
                        .unwrap()
                        * r.weight
                })
                .sum(),
        ]
    }

    pub fn get_all_feedback(&self) -> Vec<T> {
        self.readers
            .iter()
            .map(|rs| {
                rs.iter()
                    .map(|r| {
                        *self.history[r.at_node].get(-(r.at_delay as isize)).unwrap() * r.weight
                    })
                    .sum()
            })
            .collect()
    }

    pub fn get_feedback_rk4(&self, into_node: usize) -> [T; 2] {
        [
            self.readers[into_node]
                .iter()
                .map(|r| *self.history[r.at_node].get(-(r.at_delay as isize)).unwrap() * r.weight)
                .sum(),
            self.readers[into_node]
                .iter()
                .map(|r| {
                    *self.history[r.at_node]
                        .get(1 - (r.at_delay as isize))
                        .unwrap()
                        * r.weight
                })
                .sum(),
        ]
        // [T::default(), T::default()]
    }

    pub fn get_all_feedback_rk4(&self) -> Vec<[T; 2]> {
        self.readers
            .iter()
            .map(|rs| {
                [
                    rs.iter()
                        .map(|r| {
                            *self.history[r.at_node].get(-(r.at_delay as isize)).unwrap() * r.weight
                        })
                        .sum(),
                    rs.iter()
                        .map(|r| {
                            *self.history[r.at_node]
                                .get(1 - (r.at_delay as isize))
                                .unwrap()
                                * r.weight
                        })
                        .sum(),
                ]
            })
            .collect()
    }

    pub fn push_node_states(&mut self, new_states: Vec<T>) {
        for (h, s) in self.history.iter_mut().zip(new_states) {
            h.push(s)
        }
    }

    pub fn push_node_state(&mut self, node: usize, new_state: T) {
        self.history[node].push(new_state)
    }

    fn edges_valid(nodes: usize, edges: &Vec<Vec<Edge>>) -> Result<(), String> {
        for es in edges {
            for e in es {
                if e.from >= nodes {
                    return Err("edge from nonexistent node".to_string());
                } else if e.delay == 0.0 {
                    return Err("delay too small - cannot use with runge kutta 4".to_string());
                }
            }
        }
        Ok(())
    }

    fn max_delay_steps_needed(
        nodes: usize,
        edges: &Vec<Vec<Edge>>,
        use_equal_ringbuffers: bool, // test
    ) -> Vec<f64> {
        // check if edges are well formed, might be improved.
        for es in edges {
            for e in es {
                if e.from >= nodes {
                    panic!("edge from nonexistent node");
                } else if e.delay == 0.0 {
                    panic!("delay too small - cannot use with runge kutta 4");
                }
            }
        }

        let longest_needed_delays: Vec<f64> = (0..nodes)
            .map(|n| {
                edges
                    .iter()
                    .flatten()
                    .filter_map(|e| if e.from == n { Some(e.delay) } else { None })
                    .fold(0.0f64, |max_delay, delay| max_delay.max(delay))
            })
            .collect();

        if !use_equal_ringbuffers {
            let longest_global_delay = longest_needed_delays
                .iter()
                .fold(0.0f64, |max_delay, &delay| max_delay.max(delay));
            vec![longest_global_delay; nodes]
        } else {
            longest_needed_delays
        }
    }

    fn initialize_history(fill_value: Option<T>, history: &mut Vec<AllocRingBuffer<T>>) {
        match fill_value {
            Some(value) => {
                for h in history {
                    h.fill(value)
                }
            }
            None => {
                for h in history {
                    h.fill_default()
                }
            }
        }
    }
}

impl<S, T> Default for History<S, T>
where
    S: Feedback,
    T: Sized
        + Clone
        + Copy
        + Default
        + core::iter::Sum
        + std::ops::Add<Output = T>
        + std::ops::AddAssign
        + std::ops::Mul<S::WeightT, Output = T>,
{
    fn default() -> Self {
        History {
            history: Vec::new(),
            readers: Vec::new(),
            dt: 1.0 / 64.0,
        }
    }
}

#[allow(dead_code)]
impl<S, T> fmt::Display for History<S, T>
where
    S: Feedback,
    T: Sized
        + Clone
        + Copy
        + Default
        + core::iter::Sum
        + std::ops::Add<Output = T>
        + std::ops::AddAssign
        + std::ops::Mul<S::WeightT, Output = T>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "`history` has {} ringbuffers, each node's ringbuffer: ",
            &self.history.len()
        )
        .unwrap();
        for (i, buffer) in self.history.iter().enumerate() {
            writeln!(
                f,
                "node {}: {:+e} entries = {} bytes",
                i,
                buffer.len(),
                buffer.len() * mem::size_of::<T>()
            )
            .unwrap();
        }
        for (i, rs) in self.readers.iter().enumerate() {
            writeln!(f, "node {} reads data from buffers: ", i).unwrap();
            for r in rs {
                writeln!(f, "     {}, with delay: {} steps", &r.at_node, &r.at_delay).unwrap();
            }
        }
        Ok(())
    }
}
