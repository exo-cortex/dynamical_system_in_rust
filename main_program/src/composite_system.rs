use crate::{
    dynamical_system::{Feedback, IntoString},
    history::History,
    integration_methods::{self, IntegrateAndWrite, IntegrationMethods, RungeKuttaDelay},
    network::Network,
};

const SEGMENT_SIZE: usize = 1024;

#[allow(dead_code)]
pub enum NodeSetup {
    Single,
    Identical,
    Distinct,
}

pub fn new_composite_system<'a, DynSystemT>(
    network: &'a Network,
    dt: f64,
    node_setup: NodeSetup,
) -> Box<dyn IntegrationMethods>
where
    DynSystemT: Feedback + 'static,
{
    match (network.get_nodes(), node_setup) {
        (1, _) => Box::new(SingleFeedbackSystem::<DynSystemT>::new(&network, dt)),
        (2.., NodeSetup::Identical) => Box::new(
            MultipleIdenticalFeedbackSystems::<DynSystemT>::new(&network, dt),
        ),

        (2.., NodeSetup::Distinct) => Box::new(MultipleDistinctFeedbackSystems::<DynSystemT>::new(
            &network, dt,
        )),
        (_, _) => unreachable!(),
    }
}

#[derive(Default)]
#[allow(dead_code)]
pub struct SingleFeedbackSystem<DynSystemT>
where
    DynSystemT: Feedback,
{
    dt: f64,
    time: f64,
    state: DynSystemT::StateT,
    model: DynSystemT::ModelT,
    feedback_history: History<DynSystemT, RungeKuttaDelay<DynSystemT::FeedbackT>>,
    // segment: Vec<DynSystemT::KeepT>,
}

#[allow(dead_code)]
impl<DynSystemT> SingleFeedbackSystem<DynSystemT>
where
    DynSystemT: Feedback + 'static,
{
    pub fn new(network: &Network, dt: f64) -> Self {
        SingleFeedbackSystem {
            dt,
            time: 0.0,
            state: DynSystemT::StateT::default(),
            model: DynSystemT::ModelT::default(),
            feedback_history: History::<DynSystemT, RungeKuttaDelay<DynSystemT::FeedbackT>>::new(
                dt, &network, true,
            ),
            // segment: vec![DynSystemT::KeepT::default(); SEGMENT_SIZE],
        }
    }
}

#[allow(dead_code)]
impl<DynSystemT> IntegrationMethods for SingleFeedbackSystem<DynSystemT>
where
    DynSystemT: Feedback,
{
    fn single_step_rk4(&mut self) {
        let mut keep_for_feedback = RungeKuttaDelay::<DynSystemT::FeedbackT>::default();
        integration_methods::rk4_delay::<DynSystemT>(
            &mut self.state,
            &self.model,
            &mut keep_for_feedback,
            &self.feedback_history.get_single_node_feedback_rk4(),
            &self.dt,
            DynSystemT::f,
            DynSystemT::get_feedback,
        );
        self.feedback_history.push_node_state(0, keep_for_feedback);
        self.time += self.dt;
    }

    fn n_steps_rk4(&mut self, n: usize) {
        for _ in 0..n {
            self.single_step_rk4();
        }
    }

    fn into_str(&self) -> String {
        format!("{}\t{}", self.time, self.state.write_out())
    }
}

impl<DynSystemT> IntegrateAndWrite for SingleFeedbackSystem<DynSystemT>
where
    DynSystemT: Feedback,
{
    type KeepT = Vec<DynSystemT::KeepT>;
    fn keep(&self) -> Self::KeepT {
        vec![DynSystemT::keep_state(&self.state)]
    }
    // fn integrate_and_write_segment(&mut self) {}
}

// // ++++++++++++++++++++++++++++++++

#[derive(Default)]
#[allow(dead_code)]
pub struct MultipleIdenticalFeedbackSystems<DynSystemT>
where
    DynSystemT: Feedback + 'static,
{
    dt: f64,
    time: f64,
    nodes: usize,
    states: Vec<DynSystemT::StateT>,
    model: DynSystemT::ModelT,
    feedback_history: History<DynSystemT, RungeKuttaDelay<DynSystemT::FeedbackT>>,
    // segment: Vec<Vec<DynSystemT::KeepT>>,
}

#[allow(dead_code)]
impl<DynSystemT> MultipleIdenticalFeedbackSystems<DynSystemT>
where
    DynSystemT: Feedback,
{
    pub fn new(network: &Network, dt: f64) -> Self {
        MultipleIdenticalFeedbackSystems {
            dt,
            time: 0.0,
            nodes: network.get_nodes(),
            states: vec![DynSystemT::StateT::default(); network.get_nodes()],
            model: DynSystemT::ModelT::default(),
            feedback_history: History::<DynSystemT, RungeKuttaDelay<DynSystemT::FeedbackT>>::new(
                dt, &network, true,
            ),
            // segment: vec![vec![DynSystemT::KeepT::default(); network.get_nodes()]; SEGMENT_SIZE],
        }
    }
}

#[allow(dead_code)]
impl<DynSystemT> IntegrationMethods for MultipleIdenticalFeedbackSystems<DynSystemT>
where
    DynSystemT: Feedback,
{
    fn single_step_rk4(&mut self) {
        let mut keep_for_feedback =
            vec![RungeKuttaDelay::<DynSystemT::FeedbackT>::default(); self.nodes];
        for ((mut s, mut k), f) in &mut self
            .states
            .iter_mut()
            .zip(&mut keep_for_feedback)
            .zip(&self.feedback_history.get_all_feedback_rk4())
        {
            integration_methods::rk4_delay::<DynSystemT>(
                &mut s,
                &self.model,
                &mut k,
                f,
                &self.dt,
                DynSystemT::f,
                DynSystemT::get_feedback,
            );
        }
        self.feedback_history.push_node_states(keep_for_feedback);
        self.time += self.dt;
    }

    fn n_steps_rk4(&mut self, n: usize) {
        for _ in 0..n {
            self.single_step_rk4();
        }
    }

    fn into_str(&self) -> String {
        format!(
            "{}\t{}",
            self.time,
            self.states.iter().fold("".to_string(), |acc, s| format!(
                "{}\t{}",
                acc,
                &s.write_out()
            ))
        )
        .to_owned()
    }
}

impl<DynSystemT> IntegrateAndWrite for MultipleIdenticalFeedbackSystems<DynSystemT>
where
    DynSystemT: Feedback,
{
    type KeepT = Vec<DynSystemT::KeepT>;
    fn keep(&self) -> Self::KeepT {
        self.states
            .iter()
            .map(|s| DynSystemT::keep_state(s))
            .collect::<Self::KeepT>()
    }
}

// // ++++++++++++++++++++++++++++++++

#[derive(Default)]
#[allow(dead_code)]
pub struct MultipleDistinctFeedbackSystems<DynSystemT>
where
    DynSystemT: Feedback,
{
    dt: f64,
    time: f64,
    nodes: usize,
    states: Vec<DynSystemT::StateT>,
    models: Vec<DynSystemT::ModelT>,
    feedback_history: History<DynSystemT, RungeKuttaDelay<DynSystemT::FeedbackT>>,
    // segment: Vec<Vec<DynSystemT::KeepT>>,
}

#[allow(dead_code)]
impl<DynSystemT> MultipleDistinctFeedbackSystems<DynSystemT>
where
    DynSystemT: Feedback + 'static,
{
    pub fn new(network: &Network, dt: f64) -> Self {
        MultipleDistinctFeedbackSystems {
            dt,
            time: 0.0,
            nodes: network.get_nodes(),
            states: vec![DynSystemT::StateT::default(); network.get_nodes()],
            models: vec![DynSystemT::ModelT::default(); network.get_nodes()],
            feedback_history: History::<DynSystemT, RungeKuttaDelay<DynSystemT::FeedbackT>>::new(
                dt, &network, true,
            ),
            // segment: vec![vec![DynSystemT::KeepT::default(); network.get_nodes()]; SEGMENT_SIZE],
        }
    }
}

#[allow(dead_code)]
impl<DynSystemT> IntegrationMethods for MultipleDistinctFeedbackSystems<DynSystemT>
where
    DynSystemT: Feedback,
{
    fn single_step_rk4(&mut self) {
        let mut keep_for_feedback =
            vec![RungeKuttaDelay::<DynSystemT::FeedbackT>::default(); self.nodes];
        for (((s, m), f), k) in &mut self
            .states
            .iter_mut()
            .zip(&self.models)
            .zip(&self.feedback_history.get_all_feedback_rk4())
            .zip(&mut keep_for_feedback)
        {
            integration_methods::rk4_delay::<DynSystemT>(
                s,
                m,
                k,
                f,
                &self.dt,
                DynSystemT::f,
                DynSystemT::get_feedback,
            );
        }
        self.feedback_history.push_node_states(keep_for_feedback);
        self.time += self.dt;
    }

    fn n_steps_rk4(&mut self, n: usize) {
        for _ in 0..n {
            self.single_step_rk4();
        }
    }

    fn into_str(&self) -> String {
        format!(
            "{}\t{}",
            self.time,
            self.states.iter().fold("".to_string(), |acc, s| format!(
                "{}\t{}",
                acc,
                s.write_out()
            ))
        )
        .to_owned()
    }
}

impl<DynSystemT> IntegrateAndWrite for MultipleDistinctFeedbackSystems<DynSystemT>
where
    DynSystemT: Feedback,
{
    type KeepT = Vec<DynSystemT::KeepT>;
    fn keep(&self) -> Self::KeepT {
        self.states
            .iter()
            .map(|s| DynSystemT::keep_state(s))
            .collect::<Self::KeepT>()
    }
}
