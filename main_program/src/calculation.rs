use {
    network_builder::network::Network,
    segment_storage::{all_timeseries, SegmentStorage},
};

use crate::{
    composite_system::{
        MultipleDistinctFeedbackSystems, MultipleIdenticalFeedbackSystems, SingleFeedbackSystem,
    },
    integration_methods::IntegrationMethods,
};

use dynamical_systems::{
    fitzhugh_nagumo, hindmarsh_rose, lang_kobayashi, lorenz, mackey_glass, roessler, stuart_landau,
};

#[allow(dead_code)]
pub enum Tasks {
    IntegrateUntilTimeNoSave { time: f64 },
    IntegrateSegmentsAndSave { segments: usize, epsilon: f64 },
    PrintTechnicalDetails,
}

#[allow(dead_code)]
pub struct Calculation<'a, 'b> {
    dt: f64,
    time: f64,
    pub total_steps: u64,
    pub segment_length: usize,
    network: &'a Network,
    pub system: Box<dyn IntegrationMethods>,
    pub segment_storage: SegmentStorage,
    task_sequence: &'b Vec<Tasks>,
}

#[allow(dead_code)]
impl<'a, 'b> Calculation<'a, 'b> {
    pub fn single_step_rk4(&mut self) {
        self.system.single_step_rk4();
    }
    pub fn single_step_rk4_count(&mut self) {
        self.system.single_step_rk4();
        self.total_steps += 1;
    }
    pub fn n_steps_rk4(&mut self, n: usize) {
        self.system.n_steps_rk4(n);
        self.total_steps += n as u64;
    }
    pub fn integrate_segment(&mut self) {
        self.system
            .integrate_and_keep_segment(&mut self.segment_storage);
        self.total_steps += self.segment_length as u64;
    }

    pub fn perform_tasks(&mut self) {
        for task in self.task_sequence {
            match task {
                Tasks::IntegrateUntilTimeNoSave { time } => {
                    let time_in_steps = (time / self.dt) as usize;
                    self.n_steps_rk4(time_in_steps);
                }
                Tasks::IntegrateSegmentsAndSave { segments, epsilon } => {
                    for _ in 0..*segments {
                        self.system
                            .integrate_and_keep_segment(&mut self.segment_storage);
                        self.segment_storage.simplify_and_save(*epsilon);
                        self.total_steps += self.segment_length as u64;
                    }
                }
                Tasks::PrintTechnicalDetails => {
                    self.segment_storage.display_simplification_ratio();
                }
                _ => {
                    todo!();
                }
            }
        }
    }

    pub fn examples(
        dt: f64,
        network: &'a Network,
        segment_length: usize,
        node_setup: NodeSetup,
        system_type: SystemType,
        task_sequence: &'b Vec<Tasks>,
    ) -> Self {
        let system = new_composite_system_of_type(network, dt, node_setup, system_type);

        let num_nodes = network.get_nodes();
        let curve_names = system.timeseries_curve_names();
        let total_columns = num_nodes * system.timeseries_row_len();
        let output_curves = all_timeseries(num_nodes, curve_names);

        let segment_storage =
            SegmentStorage::new(dt, total_columns, segment_length, output_curves);

        Calculation {
            dt,
            time: 0.0,
            total_steps: 0,
            segment_length,
            network,
            system,
            segment_storage,
            task_sequence,
        }
    }
}

#[allow(dead_code)]
pub enum NodeSetup {
    Single,
    Identical,
    Distinct,
}

#[allow(dead_code)]
pub enum SystemType {
    LangKobayashi,
    Lorenz,
    MackeyGlass,
    HindmarshRose,
    StuartLandau,
    FitzHughNagumo,
    Roessler,
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
                SystemType::Lorenz => {
                    println!("Lorenz");
                    Box::new(SingleFeedbackSystem::<lorenz::System>::new(&network, dt))
                }
                SystemType::HindmarshRose => {
                    println!("Hindmarsh-Rose");
                    Box::new(SingleFeedbackSystem::<hindmarsh_rose::System>::new(
                        &network, dt,
                    ))
                }
                SystemType::StuartLandau => {
                    println!("Stuart-Landau");
                    Box::new(SingleFeedbackSystem::<stuart_landau::System>::new(
                        &network, dt,
                    ))
                }
                SystemType::FitzHughNagumo => {
                    println!("FitzHugh-Nagumo");
                    Box::new(SingleFeedbackSystem::<fitzhugh_nagumo::System>::new(
                        &network, dt,
                    ))
                }
                SystemType::Roessler => {
                    println!("Roessler");
                    Box::new(SingleFeedbackSystem::<roessler::System>::new(&network, dt))
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
                SystemType::Lorenz => {
                    println!("Lorenz");
                    Box::new(MultipleIdenticalFeedbackSystems::<lorenz::System>::new(
                        &network, dt,
                    ))
                }
                SystemType::HindmarshRose => {
                    println!("Hindmarsh-Rose");
                    Box::new(
                        MultipleIdenticalFeedbackSystems::<hindmarsh_rose::System>::new(
                            &network, dt,
                        ),
                    )
                }
                SystemType::StuartLandau => {
                    println!("Stuart-Landau");
                    Box::new(
                        MultipleIdenticalFeedbackSystems::<stuart_landau::System>::new(
                            &network, dt,
                        ),
                    )
                }
                SystemType::FitzHughNagumo => {
                    println!("FitzHugh-Nagumo");
                    Box::new(
                        MultipleIdenticalFeedbackSystems::<fitzhugh_nagumo::System>::new(
                            &network, dt,
                        ),
                    )
                }
                SystemType::Roessler => {
                    println!("Roessler");
                    Box::new(MultipleIdenticalFeedbackSystems::<roessler::System>::new(
                        &network, dt,
                    ))
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
                SystemType::Lorenz => {
                    println!("Lorenz");
                    Box::new(MultipleDistinctFeedbackSystems::<lorenz::System>::new(
                        &network, dt,
                    ))
                }
                SystemType::HindmarshRose => {
                    println!("Hindmarsh-Rose");
                    Box::new(
                        MultipleDistinctFeedbackSystems::<hindmarsh_rose::System>::new(
                            &network, dt,
                        ),
                    )
                }
                SystemType::StuartLandau => {
                    println!("Stuart-Landau");
                    Box::new(
                        MultipleDistinctFeedbackSystems::<stuart_landau::System>::new(&network, dt),
                    )
                }
                SystemType::FitzHughNagumo => {
                    println!("FitzHugh-Nagumo");
                    Box::new(
                        MultipleDistinctFeedbackSystems::<fitzhugh_nagumo::System>::new(
                            &network, dt,
                        ),
                    )
                }
                SystemType::Roessler => {
                    println!("Roessler");
                    Box::new(MultipleDistinctFeedbackSystems::<roessler::System>::new(
                        &network, dt,
                    ))
                }
            }
        }
        (_, _) => unreachable!(),
    }
}
