// -write simplification function for parameteric plots where samples are not equidistant in time
// -for n-dimensional systems ( state(t) = (x_1(t), ..., x_n(t))):
//  make simplification based on each curve
//  make simplification based on all together.
// >>> https://en.wikipedia.org/wiki/Distance_from_a_point_to_a_line#Vector_formulation (n-dimensional vector formulation)
// >>> with x = a + t*n (a, n element of V^n)
// >>> distance = norm((p-a) - ((p - a) * n)n)

// extern crate derive_more;
// use derive_more::{Add, Mul}; // AddAssign, MulAssign, Div

use std::fs::File;
use std::io::{BufWriter, Write};

#[allow(dead_code)]
pub fn distance_point_to_line_squared(a: &Vec<f64>, b: &Vec<f64>, point: &Vec<f64>) -> f64 {
	// -> compare to here: https://softwareengineering.stackexchange.com/questions/168572/distance-from-point-to-n-dimensional-line
	// n_vector pa = P - A
	// n_vector ba = B - A
	// double t = dot(pa, ba)/dot(ba, ba)
	// double d = length(pa - t * ba)
	if a.len() != b.len() || a.len() != point.len() {
		panic!("vectors lengths are not equal!");
	}

	let paba: f64 = point
		.iter()
		.zip(a)
		.zip(b)
		.map(|((p, a), b)| (p - a) * (b - a))
		.sum();
	let baba: f64 = a.iter().zip(b).map(|(a, b)| (b - a) * (b - a)).sum();
	let t = paba / baba;
	let squared_distance = a
		.iter()
		.zip(b)
		.zip(point)
		.map(|((a, b), p)| ((p - a) - t * (b - a)).powi(2))
		.sum::<f64>();
	squared_distance
}

// do rdp (ramer-douglas-peucker) curve simplification
// assumption: a timeseries has x values in an ascending, equidistant order

pub fn simplified_write(curve: &Vec<Vec<f64>>, epsilon: f64, outfile: &mut BufWriter<File>) -> () {
	simplify_parametric_curve(curve, 0, curve.len() - 1, epsilon.sqrt(), outfile);
	write_row(outfile, &curve.last().unwrap());
}

// simplify one n-dimensional curve, use n-dimensional point-to-line-distance
fn simplify_parametric_curve(
	curve: &Vec<Vec<f64>>,
	first: usize,
	last: usize,
	epsilon_square: f64,
	outfile: &mut BufWriter<File>,
) {
	let mut max_sqr_distance = 0.0;
	let mut index_of_max: usize = first + 1;
	for i in first + 1..last {
		let sqr_d = distance_point_to_line_squared(&curve[first], &curve[last], &curve[i]);
		if max_sqr_distance < sqr_d {
			max_sqr_distance = sqr_d;
			index_of_max = i;
		}
	}
	if max_sqr_distance > epsilon_square {
		simplify_parametric_curve(&curve, first, index_of_max, epsilon_square, outfile);
		simplify_parametric_curve(&curve, index_of_max, last, epsilon_square, outfile);
		return;
	} else {
		write_row(outfile, &curve[index_of_max]);
	}
	return;
}

// this shall be used to simplify (and write out) individual timeseries for each dynamic variable
// for f(t) = (x_0(t), x_1(t), ..., x_n(t)) it shall write out
// {{t, x_0},{t, x_1}, ... , {t, x_n}}
#[allow(dead_code)]
pub fn simplified_subset_timeseries(
	curve: &Vec<Vec<f64>>,
	epsilon: f64,
	outfiles: &mut Vec<BufWriter<File>>,
) -> () {
	for i in 1..curve[0].len() {
		let indices = vec![0, i];
		simplify_subset_curve(
			curve,
			&indices,
			0,
			curve.len() - 1,
			epsilon.sqrt(),
			&mut outfiles[i],
		);
		write_row(&mut outfiles[i], &curve.last().unwrap());
	}
}

#[allow(dead_code)]
fn simplify_subset_curve(
	curve: &Vec<Vec<f64>>,
	indices: &Vec<usize>, // can only have 2 values (right now)
	first: usize,
	last: usize,
	epsilon_square: f64,
	outfile: &mut BufWriter<File>,
) {
	let mut max_sqr_distance = 0.0;
	let mut index_of_max: usize = first + 1;
	let a = vec![curve[first][indices[0]], curve[first][indices[1]]];
	let b = vec![curve[last][indices[0]], curve[last][indices[1]]];
	for i in first + 1..last {
		let point = vec![curve[i][indices[0]], curve[i][indices[1]]];
		let sqr_d = distance_point_to_line_squared(&a, &b, &point);
		if max_sqr_distance < sqr_d {
			max_sqr_distance = sqr_d;
			index_of_max = i;
		}
	}
	if max_sqr_distance > epsilon_square {
		simplify_parametric_curve(&curve, first, index_of_max, epsilon_square, outfile);
		simplify_parametric_curve(&curve, index_of_max, last, epsilon_square, outfile);
		return;
	} else {
		write_row(outfile, &curve[index_of_max]);
	}
	return;
}

fn write_row(outfile: &mut BufWriter<File>, values: &[f64]) {
	for i in 0..values.len() - 1 {
		let value_string = format!("{:.20}\t", &values[i]);
		write!(outfile, "{}", value_string).unwrap();
	}
	let value_string = format!("{:.20}\n", &values.last().unwrap());
	write!(outfile, "{}", value_string).unwrap();
}
