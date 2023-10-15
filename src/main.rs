use std::{fs::File, io::BufWriter};

mod calculation;
mod composite_system;
mod dynamical_system;
mod history;
mod integration_methods;
mod lang_kobayashi;
mod mackey_glass;
mod network;

mod curve_simplification;

fn main() {
    let file = File::create("./test.txt").unwrap();
    let mut writer = BufWriter::new(file);

    let mut calculation = calculation::Calculation::example_setup(4);

    calculation.n_steps_rk4(100);

    for _ in 0..3000 {
        // this should be done in a single step
        for _ in 0..50 {
            calculation.single_step_rk4();
        }
        calculation.write_out(&mut writer);
    }
}
