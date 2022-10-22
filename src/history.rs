use ringbuffer::{AllocRingBuffer, RingBuffer, RingBufferExt, RingBufferWrite};
use std::fmt;
use std::mem;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct ReadAtMultiply<F>
where
    F: Sized + Clone,
{
    pub at_node: usize,
    pub at_delay: usize,
    pub weight: F,
}

#[allow(dead_code)]
impl<F> ReadAtMultiply<F>
where
    F: Sized + Clone,
{
    pub fn new(at_node: usize, at_delay: usize, weight: F) -> Self {
        ReadAtMultiply {
            at_node,
            at_delay,
            weight,
        }
    }
}

#[allow(dead_code)]
pub struct History<D, F>
// D - delay type,
// F float type for multiplication of delay in weighted sum.
where
    D: Sized
        + Clone
        + Copy
        + Default
        + core::iter::Sum
        + std::ops::Add<Output = D>
        + std::ops::AddAssign
        + std::ops::Mul<F, Output = D>
        + std::ops::Mul<F>,
    F: Sized + Clone + Copy,
{
    history: Vec<AllocRingBuffer<D>>,
    readers: Vec<Vec<ReadAtMultiply<F>>>,
}

#[allow(dead_code)]
impl<D, F> History<D, F>
where
    D: Sized
        + Clone
        + Copy
        + Default
        + core::iter::Sum
        + std::ops::Add<Output = D>
        + std::ops::AddAssign
        + std::ops::Mul<F, Output = D>
        + std::ops::Mul<f64, Output = D>
        + std::ops::Mul<F>,
    F: Sized + Clone + Copy,
{
    pub fn new(
        nodes: usize,
        edges: &[Vec<ReadAtMultiply<F>>],
        use_equal_ringbuffers: bool, // test if unequal ringbuffers are faster or same are faster
    ) -> Self {
        Self::edges_valid(nodes, &edges).unwrap();
        let mut history: Vec<AllocRingBuffer<D>> =
            Self::delay_steps(nodes, edges, use_equal_ringbuffers)
                .iter()
                .map(|delay_steps| {
                    AllocRingBuffer::<D>::with_capacity(delay_steps.next_power_of_two())
                })
                .collect();

        Self::initialize_history(None, &mut history);

        History {
            history,
            readers: edges.to_owned(),
        }
    }

    pub fn get_node_feedback(&mut self, into: usize) -> D {
        self.readers[into]
            .iter()
            .map(|r| *self.history[r.at_node].get(-(r.at_delay as isize)).unwrap() * r.weight)
            .sum()
    }

    pub fn get_node_feedback_rk4(&mut self, into: usize) -> [D; 2] {
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

    pub fn get_all_feedback(&self) -> Vec<D> {
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

    pub fn get_all_feedback_rk4(&self) -> Vec<[D; 2]> {
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

    pub fn push_node_states(&mut self, new_states: Vec<D>) {
        for (h, s) in self.history.iter_mut().zip(new_states) {
            h.push(s)
        }
    }
    fn edges_valid(nodes: usize, edges: &[Vec<ReadAtMultiply<F>>]) -> Result<(), String> {
        for es in edges {
            for e in es {
                if e.at_node >= nodes {
                    return Err("edge from nonexistent node".to_string());
                } else if e.at_delay == 0 {
                    return Err("delay too small - cannot use with runge kutta 4".to_string());
                }
            }
        }
        Ok(())
    }

    fn delay_steps(
        nodes: usize,
        edges: &[Vec<ReadAtMultiply<F>>],
        use_equal_ringbuffers: bool, // test
    ) -> Vec<usize> {
        // check if edges are well formed, might be improved.
        for es in edges {
            for e in es {
                if e.at_node >= nodes {
                    panic!("edge from nonexistent node");
                } else if e.at_delay == 0 {
                    panic!("delay too small - cannot use with runge kutta 4");
                }
            }
        }

        let mut longest_delay_at_node: Vec<usize> = (0..nodes)
            .map(|n| {
                edges
                    .iter()
                    .flatten()
                    .filter_map(|e| {
                        if e.at_node == n {
                            // println!(".");
                            Some(e.at_delay)
                        } else {
                            None
                        }
                    })
                    .fold(0, |max_delay, delay| max_delay.max(delay))
            })
            .collect();

        if use_equal_ringbuffers {
            let longest_global_delay = longest_delay_at_node.iter().max().unwrap();
            longest_delay_at_node = vec![*longest_global_delay; nodes];
        }
        longest_delay_at_node
    }

    fn initialize_history(fill_value: Option<D>, history: &mut Vec<AllocRingBuffer<D>>) {
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

impl<D, F> fmt::Display for History<D, F>
where
    D: Sized
        + Clone
        + Copy
        + Default
        + core::iter::Sum
        + std::ops::Add<Output = D>
        + std::ops::AddAssign
        + std::ops::Mul<F, Output = D>
        + std::ops::Mul<f64, Output = D>
        + std::ops::Mul<F>,
    F: Sized + Clone + Copy,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "history has {} ringbuffers, ringbuffer sizes: ",
            &self.history.len()
        )
        .unwrap();
        for (i, buffer) in self.history.iter().enumerate() {
            writeln!(
                f,
                "[{}: {:+e} entries = {} bytes]",
                i,
                buffer.len(),
                buffer.len() * mem::size_of::<D>()
            )
            .unwrap();
        }
        for (i, rs) in self.readers.iter().enumerate() {
            write!(f, "{}", i).unwrap();
            for r in rs {
                writeln!(f, " <-- {}, delay: {}", &r.at_node, &r.at_delay).unwrap();
            }
        }
        Ok(())
    }
}
