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
		beta: 8.0/3.0,
	};
	
	let integration_steps = 1000;
	let dt = 1.0 / 128.0;
	let mut time: f64 = 0.0;
	let mut outfile = BufWriter::new(File::create("lorenz_data_rk4.txt")?);

	for _ in 0..integration_steps {
		lorenz::update_rk4(&mut state, &parameters, dt);
		time += dt;
		let outline = format!("{0:.8}\t{1:.8}\t{2:.8}\t{3:.8}\n", time, state.x, state.y, state.z);
		write!(&mut outfile, "{}", outline).expect("writing into file not possible.");
	}

	println!("state at end of integration: {:?}", state);
	outfile.flush()?;
	Ok(())
}
