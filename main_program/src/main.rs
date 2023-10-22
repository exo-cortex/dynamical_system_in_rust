use core::time;
use std::{fs::File, io::BufWriter};

mod calculation;
mod composite_system;
mod dynamical_system;
mod history;
mod integration_methods;
mod lang_kobayashi;
mod mackey_glass;
mod network;

// mod curve_simplification;
// mod timeseries;

fn main() {
    let file = File::create("./test.txt").unwrap();
    let mut writer = BufWriter::new(file);

    let mut calculation = calculation::Calculation::example_setup(1);

    calculation.n_steps_rk4(0);

    for _ in 0..150000 {
        // this should be done in a single step
        for _ in 0..20 {
            calculation.single_step_rk4();
        }
        calculation.write_out(&mut writer);
    }

    // let ts = timeseries::Timeseries::new(
    //     0.1,
    //     4,
    //     3,
    //     10,
    //     vec!["a".to_owned(), "b".to_owned(), "c".to_owned()],
    // );
}
