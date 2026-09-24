use std::env;

use calculation::{NodeSetup, SystemType, Tasks};
use network_builder::network::Network;

mod timer;

mod calculation;
mod composite_system;
mod history;
mod integration_methods;

use timer::Timer;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let mut conf = Config::default();
    conf.from_args(&args);

    let mut network = Network::new(conf.n_real, 0.1, 0.1, 100.0);
    network.put_edge(2, 0, conf.strength, 0.2, conf.tau * 0.01);
    network.put_ring(conf.strength, 0.5, conf.tau);
    // network.randomize_delay_relative(0.3, network::SelectGroup::AllGroups);

    let task_sequence = vec![
        Tasks::IntegrateUntilTimeNoSave {
            time: conf.buffer_time,
        },
        Tasks::IntegrateSegmentsAndSave {
            segments: conf.segments,
            epsilon: conf.epsilon,
        },
        Tasks::PrintTechnicalDetails,
    ];

    let mut calculation = calculation::Calculation::examples(
        1.0 / conf.inv_dt,
        &network,
        conf.seg_length,
        NodeSetup::Identical,
        SystemType::HindmarshRose,
        &task_sequence,
    );

    let timer = Timer::new();
    calculation.perform_tasks();
    println!(
        "integrated {} steps in {} ms",
        calculation.total_steps,
        timer.get_nanoseconds() as f64 / 1000000.0
    );
}

struct Config {
    n_real: usize,
    strength: f64,
    inv_dt: f64,
    buffer_time: f64,
    seg_length: usize,
    segments: usize,
    epsilon: f64,
    tau: f64,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            n_real: 1,
            strength: 0.1,
            inv_dt: 512.0,
            buffer_time: 0.0,
            seg_length: 4096,
            segments: 25,
            epsilon: 0.005,
            tau: 10.0,
        }
    }
}

impl Config {
    fn from_args(&mut self, args: &[String]) {
        for (i, pattern) in args.iter().enumerate() {
            match pattern.as_str() {
                "-n" => {
                    if args.len() - i >= 1 {
                        println!("{} {}", pattern, args[i + 1]);
                        self.n_real = args[i + 1].parse().unwrap()
                    }
                }
                "-strength" => {
                    if args.len() - i >= 1 {
                        println!("{} {}", pattern, args[i + 1]);
                        self.strength = args[i + 1].parse().unwrap()
                    }
                }
                "-idt" => {
                    if args.len() - i >= 1 {
                        println!("{} {}", pattern, args[i + 1]);
                        self.inv_dt = args[i + 1].parse().unwrap()
                    }
                }
                "-buft" => {
                    if args.len() - i >= 1 {
                        println!("{} {}", pattern, args[i + 1]);
                        self.buffer_time = args[i + 1].parse().unwrap()
                    }
                }
                "-segl" => {
                    if args.len() - i >= 1 {
                        println!("{} {}", pattern, args[i + 1]);
                        self.seg_length = args[i + 1].parse().unwrap()
                    }
                }
                "-segs" => {
                    if args.len() - i >= 1 {
                        println!("{} {}", pattern, args[i + 1]);
                        self.segments = args[i + 1].parse().unwrap()
                    }
                }
                "-epsilon" => {
                    if args.len() - i >= 1 {
                        println!("{} {}", pattern, args[i + 1]);
                        self.epsilon = args[i + 1].parse().unwrap()
                    }
                }
                "-tau" => {
                    if args.len() - i >= 1 {
                        println!("{} {}", pattern, args[i + 1]);
                        self.tau = args[i + 1].parse().unwrap()
                    }
                }
                _ => {}
            }
        }
    }
}
