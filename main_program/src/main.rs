use std::env;

use calculation::{NodeSetup, SystemType, Tasks};
use network::Network;

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
mod stuart_landau;
// mod mdre;

use timer::Timer;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let mut inv_dt = 64.0;
    let mut buffer_time = 1000.0;
    let mut seg_length = 1024;
    let mut segments = 10;
    let mut epsilon = 0.1;

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
            _ => {}
        }
    }

    let mut network = Network::new(3, 0.1, 0.1, 100.0, 0, 1.0 / inv_dt);
    network.put_edge(0, 0, 0.2, 0.5, 16.8);
    network.put_ring(0.1, 0.5, 25.8);

    let mut timer = Timer::new();

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
        SystemType::HindmarshRose,
        &task_sequence,
    );

    timer.reset();

    calculation.perform_tasks();
    println!(
        "time for integration was: {} ms",
        timer.get_nanoseconds() as f64 / 1000000.0
    );

    println!("integrated {} steps", calculation.total_steps);
}
