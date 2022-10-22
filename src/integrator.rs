// use crate::dynamical_system;
use crate::curve_simplification;
use crate::dynamical_system::DynamicalDelaySystem;
use crate::history;
use crate::lang_kobayashi;
use crate::network;
use derive_more::{Add, AddAssign, Mul, Sum};
use std::fs::File;
use std::io::{BufWriter, Write};

#[allow(dead_code)]
pub enum SideTasks {
    DoAnalysis,
    SaveSimplifiedTimeseries(f64, WhichNodes, SaveWhat),
}

#[allow(dead_code)]
pub enum WhichNodes {
    All,
    Only(usize),
    Set(Vec<usize>),
}

#[allow(dead_code)]
pub enum SaveWhat {
    StatesFull,
    StateSubset(Vec<usize>),
}

#[allow(dead_code)]
pub enum WhichSystem {
    None,
    LangKobayashi,
    // StuartLandau,
    // MackeyGlass,
    // Lorenz,
    // Roessler,
    // FitzHughNagumo,
}

// maybe this is overkill / too complicated
#[derive(Copy, Clone, Default, Add, AddAssign, Mul, Sum)]
pub struct DelayStateSlope<T>
where
    T: Copy,
{
    pub state: T,
    pub slope: T,
}

impl<T> DelayStateSlope<T>
where
    T: Copy,
{
    pub fn new(states: [T; 2]) -> DelayStateSlope<T> {
        DelayStateSlope {
            state: states[0],
            slope: states[1],
        }
    }
}

#[allow(dead_code)]
#[derive(Default)]
pub struct Integrator {
    end_time: f64,
    dt: f64,
    buffer: f64,
    segment_size: usize,
    nodes: usize,
    time: f64,
    side_tasks: Option<Vec<SideTasks>>,
    network: network::Network,
    trajectory_files: Vec<BufWriter<File>>,
}

#[allow(dead_code)]
impl Integrator {
    pub fn new(
        end_time: f64,
        dt: f64,
        buffer: f64,
        segment_size: usize,
        nodes: usize,
        side_tasks: Option<Vec<SideTasks>>,
    ) -> Self {
        let network = network::Network::new(nodes, Some(0.1), Some(0.0), Some(100.0), Some(0), dt);
        Integrator {
            end_time,
            time: 0.0,
            dt,
            buffer,
            segment_size,
            nodes,
            side_tasks,
            network,
            trajectory_files: Vec::new(),
        }
    }

    pub fn integrate_rk4(&mut self) {
        let integration_steps = self.end_time / self.dt;
        let integration_steps = integration_steps as usize;
        let num_segments = integration_steps / self.segment_size + 1;
        println!(
            "integration steps: {:+e}, in {} segments of size {}",
            integration_steps, num_segments, &self.segment_size
        );
        self.setup_network();

        // init system states
        let mut states = vec![lang_kobayashi::State::default(); self.nodes];
        for s in &mut states {
            s.e = num_complex::Complex::<f64>::new(0.1, 0.0);
            s.n = 0.1;
        }
        states[0].e = num_complex::Complex::<f64>::new(0.1, 0.0);

        // init history with edges
        let mut history = history::History::<
            DelayStateSlope<lang_kobayashi::DelayState>,
            lang_kobayashi::Weight,
        >::new(self.nodes, &self.network.get_edges_into_nodes(), false);

        println!("{}", history);

        let model = lang_kobayashi::Model {
            alpha: 1.25,
            pump: 0.05,
            t_lk: 1250.0,
        };

        let mut segment: Vec<Vec<f64>> = vec![vec![0.0; 1 + &self.nodes * 2]; self.segment_size];
        self.setup_trajectory_files("data_".to_owned());

        for _ in 0..(self.buffer / self.dt) as usize {
            let mut put_into_history: Vec<DelayStateSlope<lang_kobayashi::DelayState>> = vec![
                    DelayStateSlope::<lang_kobayashi::DelayState> {
                        state: lang_kobayashi::DelayState::default(),
                        slope: lang_kobayashi::DelayState::default()
                    };
                    self.nodes
                    ];
            let per_node_delay_sums = history.get_all_feedback_rk4();

            for i in 0..self.nodes {
                put_into_history[i] = DelayStateSlope::new(rk4_delay(
                    &mut states[i],
                    &model,
                    &per_node_delay_sums[i][0],
                    &per_node_delay_sums[i][1],
                    &self.dt,
                    lang_kobayashi::DelaySystem::f,
                    lang_kobayashi::DelaySystem::keep_delay,
                ))
            }

            history.push_node_states(put_into_history);
            self.time += self.dt;
        }

        for _segment in 0..num_segments {
            for ti in 0..self.segment_size {
                // das hier ist eine große bausstelle
                // das objekt, in das das subset an states reingeschrieben wird, bevor es in die history gepusht wird:
                let mut put_into_history: Vec<DelayStateSlope<lang_kobayashi::DelayState>> = vec![
                    DelayStateSlope::<lang_kobayashi::DelayState> {
                        state: lang_kobayashi::DelayState::default(),
                        slope: lang_kobayashi::DelayState::default()
                    };
                    self.nodes
                    ];

                // vector that stores in each element the weighted sum of all the (delayed) inputs from other nodes
                let per_node_delay_sums = history.get_all_feedback_rk4();

                for i in 0..self.nodes {
                    put_into_history[i] = DelayStateSlope::new(rk4_delay(
                        &mut states[i],
                        &model,
                        &per_node_delay_sums[i][0],
                        &per_node_delay_sums[i][1],
                        &self.dt,
                        lang_kobayashi::DelaySystem::f,
                        lang_kobayashi::DelaySystem::keep_delay,
                    ))
                }

                history.push_node_states(put_into_history);
                self.keep_segment_row(&mut segment[ti], &states);
                self.time += self.dt;
            }

            curve_simplification::write_n_simplified_curves(
                &segment,
                0.0025,
                &mut self.trajectory_files,
            );
        }
        for file in &mut self.trajectory_files {
            file.flush().unwrap();
        }
    }

    fn setup_trajectory_files(&mut self, prefix: String) {
        let per_state_dimensions = 2;
        self.trajectory_files = (0..self.nodes * per_state_dimensions)
            .into_iter()
            .map(|i| {
                let filename_e = format!("{}{:02}.txt", &prefix, i);

                BufWriter::new(File::create(filename_e).unwrap())
            })
            .collect();
    }

    fn keep_segment_row(&self, segment_row: &mut [f64], states: &[lang_kobayashi::State]) {
        segment_row[0] = self.time;
        for i in 0..self.nodes {
            segment_row[1 + i * 2] = states[i].e.norm_sqr();
            segment_row[1 + i * 2 + 1] = states[i].n;
        }
    }

    pub fn setup_network(&mut self) {
        self.network.put_ring(0.1, -0.125, 200.0);
        self.network.put_edge(0, 0, 0.025, 0.1, 50.0);
        self.network
            .randomize_delay_relative(0.5, network::SelectGroup::AllGroups);
        println!("{}", &self.network);
    }
}

// integration methods
#[allow(dead_code)]
pub fn rk4<T, P>(state: &mut T, parameters: &P, dt: &f64, f: fn(&T, &P) -> T)
where
    T: Sized
        + Copy
        + std::ops::Mul<f64, Output = T>
        + std::ops::Add<T, Output = T>
        + std::ops::AddAssign
        + std::ops::Div<f64, Output = T>,
{
    // runge kutta 4 method creates 4 "helper steps"
    let k1 = f(state, parameters);
    let k2 = f(&(*state + k1 * 0.5 * *dt), parameters);
    let k3 = f(&(*state + k2 * 0.5 * *dt), parameters);
    let k4 = f(&(*state + k3 * *dt), parameters);

    *state += (k1 + k2 * 2.0 + k3 * 2.0 + k4) / 6.0 * *dt;
}

#[allow(dead_code)]
pub fn rk4_delay<T, P, D>(
    state: &mut T,
    parameters: &P,
    // with rk4 the delays for the 3 different time_positions
    // at k1, (k2+k3) and k4 are calculated through hermite interpolations
    delay_left: &DelayStateSlope<D>,
    delay_right: &DelayStateSlope<D>,
    dt: &f64,
    f: fn(&T, &P, &D) -> T,
    d: fn(&T) -> D,
) -> [D; 2]
where
    T: Sized
        + Copy
        + std::ops::Mul<f64, Output = T>
        + std::ops::Add<T, Output = T>
        + std::ops::AddAssign
        + std::ops::Div<f64, Output = T>,
    D: Sized
        + Clone
        + Copy
        + std::ops::Add<Output = D>
        + std::ops::Sub<Output = D>
        + std::ops::Mul<f64, Output = D>,
{
    // runge kutta 4 method creates 4 "helper steps"
    let k1 = f(state, parameters, &delay_left.state);
    // the middle steps k2,k3 need a delay value not existant in the history. it is created through hermite interpolation.
    let middle = (delay_left.state + delay_right.state) * 0.5
        + (delay_left.slope - delay_right.slope) * 0.125;
    let k2 = f(&(*state + k1 * 0.5 * *dt), parameters, &middle);
    let k3 = f(&(*state + k2 * 0.5 * *dt), parameters, &middle);
    let k4 = f(&(*state + k3 * *dt), parameters, &delay_right.state);

    *state += (k1 + k2 * 2.0 + k3 * 2.0 + k4) / 6.0 * *dt;
    [d(state), d(&k1)]
}

#[allow(dead_code)]
pub fn euler<T, P>(state: &mut T, parameters: &P, dt: &f64, f: fn(&T, &P) -> T)
where
    T: Sized + Copy + std::ops::Mul<f64, Output = T> + std::ops::AddAssign,
{
    *state += f(state, parameters) * *dt;
}

#[allow(dead_code)]
pub fn euler_delay<T, P, D>(
    state: &mut T,
    parameters: &P,
    delay: &D,
    dt: &f64,
    f: fn(&T, &P, &D) -> T,
) where
    T: Sized + Copy + std::ops::Mul<f64, Output = T> + std::ops::AddAssign,
{
    *state += f(state, parameters, delay) * *dt;
}
