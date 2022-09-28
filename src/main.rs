// mod analysis;
mod curve_simplification;
mod lang_kobayashi;
mod lorenz;

// experimental
mod integrator;

use num_complex::Complex;
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() -> std::io::Result<()> {
	let mut state = lang_kobayashi::State {
		e: Complex::new(0.1, 0.0),
		n: -0.1,
	};

	let parameters = lang_kobayashi::Model {
		alpha: 0.05,
		pump: 0.25,
		t_lk: 1000.0,
	};

	let segment_size = 4096 * 64;
	let segments = 16;
	let mut segment = vec![vec![0.0; 3]; segment_size];

	let dt = 1e-3;
	let epsilon = 1e-3;
	let mut time: f64 = 0.0;
	let mut outfile = BufWriter::new(File::create("data.txt").unwrap());

	for _ in 0..segments {
		for row in segment.iter_mut().take(segment_size) {
			*row = vec![time, state.e.norm_sqr(), state.n];
			time += dt;
			integrator::update_runge_kutte_4(&mut state, &parameters, dt, lang_kobayashi::f);
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
