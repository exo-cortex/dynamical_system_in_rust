use std::fs::File;
use std::io::{BufWriter, Write};

use crate::{
    composite_system::{new_composite_system, NodeSetup},
    integration_methods::IntegrationMethods,
    lang_kobayashi,
    // mackey_glass,
    network::Network,
};

#[allow(dead_code)]
pub enum OperationMode {
    SaveSegments(usize),
    SaveExtrema,
    NoSave,
}

#[allow(dead_code)]
pub struct Calculation {
    dt: f64,
    network: Network,
    pub system: Box<dyn IntegrationMethods>,
}

#[allow(dead_code)]
impl Calculation {
    pub fn single_step_rk4(&mut self) {
        self.system.single_step_rk4()
    }
    pub fn n_steps_rk4(&mut self, n: usize) {
        self.system.n_steps_rk4(n)
    }
    pub fn write_out(&self, f: &mut BufWriter<File>) {
        write!(f, "{}\n", self.system.into_str()).unwrap()
    }
    pub fn example_setup(nodes: usize) -> Self {
        // create an example setup for testing
        // should disappear later
        let dt = 1.0 / 64.0;

        let mut network = Network::new(nodes, 0.1, 0.1, 100.0, 0, dt);
        // network.put_ring(0.1, 0.1, 50.0);
        network.put_edge(0, 0, 0.1, 0.0, 100.0);
        // network.randomize_strength(0.5, crate::network::SelectGroup::SingleGroup(0));
        // network.randomize_delay_relative(0.9, crate::network::SelectGroup::SingleGroup(0));
        println!("{}", network);
        Calculation {
            dt,
            network: Network::default(),
            system: new_composite_system::<lang_kobayashi::System>(
                &network,
                dt,
                NodeSetup::Identical,
            ),
        }
    }
}

impl Default for Calculation {
    fn default() -> Self {
        let dt = 1.0 / 64.0;
        let network = Network::default();
        Calculation {
            dt,
            network: Network::default(),
            system: new_composite_system::<lang_kobayashi::System>(
                &network,
                dt,
                NodeSetup::Identical,
            ),
        }
    }
}
