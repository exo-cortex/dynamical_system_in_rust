use std::env;

use calculation::{NodeSetup, SystemType, Tasks};
use network::Network;

use timeseries::SaveItems;

mod timer;

mod calculation;
mod composite_system;
mod dynamical_system;
mod history;
mod integration_methods;
mod network;

mod fitzhugh_nagumo;
mod hindmarsh_rose;
mod lang_kobayashi;
mod lorenz;
mod mackey_glass;
// mod mdre;
mod roessler;
mod stuart_landau;

use timer::Timer;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let mut inv_dt = 512.0;
    let mut buffer_time = 0.0;
    let mut seg_length = 1024;
    let mut segments = 25;
    let mut epsilon = 0.005;
    let mut tau = 10.0;

    for (i, pattern) in args.iter().enumerate() {
        match pattern.as_str() {
            "-idt" => {
                if args.len() - i >= 1 {
                    println!("{} {}", pattern, args[i + 1]);
                    inv_dt = args[i + 1].parse().unwrap()
                }
            }
            "-buft" => {
                if args.len() - i >= 1 {
                    println!("{} {}", pattern, args[i + 1]);
                    buffer_time = args[i + 1].parse().unwrap()
                }
            }
            "-segl" => {
                if args.len() - i >= 1 {
                    println!("{} {}", pattern, args[i + 1]);
                    seg_length = args[i + 1].parse().unwrap()
                }
            }
            "-segs" => {
                if args.len() - i >= 1 {
                    println!("{} {}", pattern, args[i + 1]);
                    segments = args[i + 1].parse().unwrap()
                }
            }
            "-epsilon" => {
                if args.len() - i >= 1 {
                    println!("{} {}", pattern, args[i + 1]);
                    epsilon = args[i + 1].parse().unwrap()
                }
            }
            "-tau" => {
                if args.len() - i >= 1 {
                    println!("{} {}", pattern, args[i + 1]);
                    tau = args[i + 1].parse().unwrap()
                }
            }

            _ => {}
        }
    }

    let mut network = Network::new(1, 0.1, 0.1, 100.0, 0, 1.0 / inv_dt);
    network.put_edge(0, 0, 0.0, 0.5, tau);
    // network.put_ring(0.125, 0.5, tau * 0.321);

    let task_sequence = vec![
        Tasks::IntegrateUntilTimeNoSave { time: buffer_time },
        Tasks::IntegrateSegmentsAndSave {
            segments: segments,
            epsilon: epsilon,
        },
        Tasks::PrintTechnicalDetails,
    ];

    let mut calculation = calculation::Calculation::examples(
        1.0 / inv_dt,
        &network,
        seg_length,
        NodeSetup::Identical,
        SystemType::Lorenz,
        &task_sequence,
        SaveItems::ParametricCurve2d {
            variable_pairs: vec![[0, 2]],
        },
    );

    let mut timer = Timer::new();
    calculation.perform_tasks();
    println!(
        "integrated {} steps in {} ms",
        calculation.total_steps,
        timer.get_nanoseconds() as f64 / 1000000.0
    );
}
