use core::time;
use std::fs::File;
use std::io::{BufWriter, Write};

use timeseries::Timeseries;

use crate::dynamical_system::{DynamicalSystem, Feedback};

use crate::{
    composite_system::{
        MultipleDistinctFeedbackSystems, MultipleIdenticalFeedbackSystems, SingleFeedbackSystem,
    },
    integration_methods::IntegrationMethods,
    lang_kobayashi, mackey_glass,
    network::Network,
};

#[allow(dead_code)]
pub enum OperationMode {
    SaveSegments(usize),
    SaveExtrema,
    NoSave,
}

#[allow(dead_code)]
pub struct Calculation<'a> {
    dt: f64,
    time: f64, // maybe time should be here ?
    pub total_steps: u64,
    pub segment_length: usize,
    network: &'a Network,
    pub system: Box<dyn IntegrationMethods>,
    pub timeseries: Timeseries,
}

#[allow(dead_code)]
impl<'a> Calculation<'a> {
    pub fn single_step_rk4(&mut self) {
        self.system.single_step_rk4()
    }
    pub fn single_step_rk4_count(&mut self) {
        self.system.single_step_rk4();
        self.total_steps += 1;
    }
    pub fn n_steps_rk4(&mut self, n: usize) {
        self.system.n_steps_rk4(n)
    }
    pub fn write_out(&self, f: &mut BufWriter<File>) {
        write!(f, "{}\n", self.system.into_str()).unwrap()
    }
    pub fn integrate_segment(&mut self) {
        self.system.integrate_and_keep_segment(&mut self.timeseries);
        self.total_steps += self.segment_length as u64;
    }
    pub fn integrate_segment_and_save(&mut self) {
        self.system.integrate_and_keep_segment(&mut self.timeseries);
        self.timeseries.save_simplified_timeseries();
        self.total_steps += self.segment_length as u64;
    }

    // +++++++++++++++++++++++++
    // +++++++++++++++++++++++++
    // +++++++++++++++++++++++++
    pub fn examples(
        dt: f64,
        network: &'a Network,
        segment_length: usize,
        node_setup: NodeSetup,
        system_type: SystemType,
    ) -> Self {
        let system = new_composite_system_of_type(&network, dt, node_setup, system_type);

        let timeseries = Timeseries::new(
            dt,
            network.get_nodes(),
            system.timeseries_row_len(),
            segment_length,
            system.timeseries_curve_names(),
        );

        Calculation {
            dt: dt,
            time: 0.0,
            total_steps: 0,
            segment_length,
            network,
            system,
            timeseries,
        }
    }

    // pub fn example_setup_lang_kobayashi(nodes: usize, dt: f64, segment_length: usize) -> Self {
    //     // create an example setup for testing

    //     let mut network = Network::new(nodes, 0.1, 0.1, 100.0, 0, dt);
    //     network.put_ring(0.1, 0.0, 512.0);
    //     network.put_ring_reverse(0.1, 0.25, 121.0);
    //     // network.put_edge(1, 1, 0.1, 0.0, 123.0);
    //     network.put_edge(0, 0, 0.1, 0.0, 400.0);
    //     network.put_edge(3, 3, 0.01, 0.0, 134.0);
    //     network.randomize_strength(0.1, crate::network::SelectGroup::AllGroups);
    //     network.randomize_delay_relative(0.1, crate::network::SelectGroup::AllGroups);
    //     // println!("{}", network);
    //     Calculation {
    //         dt,
    //         time: 0.0,
    //         total_steps: 0,
    //         segment_length,
    //         network: &network,
    //         system: new_composite_system::<lang_kobayashi::System>(
    //             &network,
    //             dt,
    //             NodeSetup::Identical,
    //         ),
    //         timeseries: Timeseries::new(
    //             dt,
    //             network.get_nodes(),
    //             lang_kobayashi::System::keep_state(&lang_kobayashi::State::default()).len(),
    //             segment_length,
    //             lang_kobayashi::System::keep_state_names(),
    //         ),
    //     }
    // }
    // pub fn example_setup_mackey(nodes: usize, dt: f64, segment_length: usize) -> Self {
    //     let mut network = Network::new(nodes, 0.1, 0.1, 100.0, 0, dt);
    //     network.put_edge(0, 0, 1.0, 0.0, 15.0);
    //     network.put_edge(4, 4, 5.0, 0.0, 125.0);
    //     network.put_ring(0.5, 0.0, 30.0);
    //     println!("{}", network);
    //     Calculation {
    //         dt,
    //         time: 0.0,
    //         total_steps: 0,
    //         segment_length,
    //         network: &network,
    //         system: new_composite_system::<mackey_glass::System>(
    //             &network,
    //             dt,
    //             NodeSetup::Identical,
    //         ),
    //         timeseries: Timeseries::new(
    //             dt,
    //             network.get_nodes(),
    //             mackey_glass::System::keep_state(&mackey_glass::State::default()).len(),
    //             segment_length,
    //             mackey_glass::System::keep_state_names(),
    //         ),
    //     }
    // }
}

#[allow(dead_code)]
pub enum NodeSetup {
    Single,
    Identical,
    Distinct,
}

pub enum SystemType {
    LangKobayashi,
    MackeyGlass,
}

pub fn new_composite_system_of_type(
    network: &Network,
    dt: f64,
    node_setup: NodeSetup,
    system_type: SystemType,
) -> Box<dyn IntegrationMethods> {
    match (network.get_nodes(), node_setup) {
        (1, _) => {
            print!("### single system of type: ");
            match system_type {
                SystemType::LangKobayashi => {
                    println!("Lang-Kobayashi");
                    Box::new(SingleFeedbackSystem::<lang_kobayashi::System>::new(
                        &network, dt,
                    ))
                }
                SystemType::MackeyGlass => {
                    println!("Mackey-Glass");
                    Box::new(SingleFeedbackSystem::<mackey_glass::System>::new(
                        &network, dt,
                    ))
                }
            }
        }
        (2.., NodeSetup::Identical) => {
            print!("### multiple identical systems of tyle: ");
            match system_type {
                SystemType::LangKobayashi => {
                    println!("Lang-Kobayashi");
                    Box::new(
                        MultipleIdenticalFeedbackSystems::<lang_kobayashi::System>::new(
                            &network, dt,
                        ),
                    )
                }
                SystemType::MackeyGlass => {
                    println!("Mackey-Glass");
                    Box::new(
                        MultipleIdenticalFeedbackSystems::<mackey_glass::System>::new(&network, dt),
                    )
                }
            }
        }

        (2.., NodeSetup::Distinct) => {
            println!("### multiple distinct systems of type: ");
            match system_type {
                SystemType::LangKobayashi => {
                    println!("Lang-Kobayashi");
                    Box::new(
                        MultipleDistinctFeedbackSystems::<lang_kobayashi::System>::new(
                            &network, dt,
                        ),
                    )
                }
                SystemType::MackeyGlass => {
                    println!("Mackey-Glass");
                    Box::new(
                        MultipleDistinctFeedbackSystems::<mackey_glass::System>::new(&network, dt),
                    )
                }
            }
        }
        (_, _) => unreachable!(),
    }
}

// pub fn new_composite_system<'a, DynSystemT>(
//     network: &'a Network,
//     dt: f64,
//     node_setup: NodeSetup,
// ) -> Box<dyn IntegrationMethods>
// where
//     DynSystemT: Feedback + 'static,
// {
//     match (network.get_nodes(), node_setup) {
//         (1, _) => {
//             println!("### single system");
//             Box::new(SingleFeedbackSystem::<DynSystemT>::new(&network, dt))
//         }
//         (2.., NodeSetup::Identical) => {
//             println!("### multiple identical systems");
//             Box::new(MultipleIdenticalFeedbackSystems::<DynSystemT>::new(
//                 &network, dt,
//             ))
//         }
//         (2.., NodeSetup::Distinct) => {
//             println!("### multiple distinct systems");
//             Box::new(MultipleDistinctFeedbackSystems::<DynSystemT>::new(
//                 &network, dt,
//             ))
//         }
//         (_, _) => unreachable!(),
//     }
// }
