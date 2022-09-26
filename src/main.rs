// mod analysis;
mod curve_simplification;
mod lorenz;

use std::fs::File;
use std::io::{BufWriter, Write};

fn main() -> std::io::Result<()> {
	let mut state: lorenz::State = lorenz::State {
		x: 1.0,
		y: 1.0,
		z: 1.0,
	};

	let parameters: lorenz::Model = lorenz::Model {
		sigma: 10.0,
		rho: 28.0,
		beta: 8.0 / 3.0,
	};

	let segment_size = 1024;
	let segments = 100;
	let dt = 1.0 / 2.0f64.powi(10);
	let epsilon = 0.00001;
	let mut time: f64 = 0.0;
	let mut outfile = BufWriter::new(File::create("lorenz_data_rk4.txt").unwrap());

	let mut segment = vec![vec![0.0; 4]; segment_size];
	for _ in 0..segments {
		for i in 0..segment_size {
			time += dt;
			lorenz::update_rk4(&mut state, &parameters, dt);
			segment[i] = vec![time, state.x, state.y, state.z];
		}
		curve_simplification::simplified_write(&segment, epsilon, &mut outfile);
	}

	outfile.flush().unwrap();
	println!(
		"integration of {} steps with stepsize dt={}. final state: {:?}.",
		segment_size * segments,
		dt,
		state
	);
	Ok(())
}
