use std::fs::File;
use std::io::{BufWriter, Write};

#[allow(dead_code)]
pub fn simplify_curves_individually(
    dt: f64,
    start_time: f64,
    curves: &Vec<Vec<f64>>,
    epsilon: f64,
    outfiles: &mut [BufWriter<File>],
) {
    for (i, outfile) in &mut outfiles.iter_mut().enumerate() {
        // write!(outfile, "{}\t{}\n", start_time as f64, &curve[0][i]).unwrap();
        recursively_simplify(
            dt,
            start_time,
            curves,
            i,
            0,
            curves.len() - 1,
            epsilon.powi(2),
            outfile,
        );
        // write_row(outfile, (curve.last().unwrap()));
        outfile.flush().unwrap();
    }
}

#[allow(dead_code)]
fn recursively_simplify(
    dt: f64,
    start_time: f64,
    curves: &Vec<Vec<f64>>,
    curve_index: usize,
    fragment_start_index: usize,
    fragment_end_index: usize,
    epsilon_square: f64,
    outfile: &mut BufWriter<File>,
) {
    let mut max_distance_sqr = 0.0;
    let mut index_of_max = fragment_start_index + 1;

    let a = [
        start_time + dt * fragment_start_index as f64,
        curves[fragment_start_index][curve_index],
    ];
    let b = [
        start_time + dt * fragment_end_index as f64,
        curves[fragment_end_index][curve_index],
    ];

    // here cargo clippy makes a suggestion
    // for i in fragment_start_index + 1..fragment_end_index {
    for (i, row) in curves
        .iter()
        .enumerate()
        .take(fragment_end_index)
        .skip(fragment_start_index + 1)
    {
        let mid_point = [start_time + dt * i as f64, row[curve_index]];

        let sqr_d = distance_point_to_line_squared_2d(a, b, mid_point);
        if max_distance_sqr < sqr_d {
            max_distance_sqr = sqr_d;
            index_of_max = i;
        }
    }
    if max_distance_sqr > epsilon_square {
        recursively_simplify(
            dt,
            start_time,
            curves,
            curve_index,
            fragment_start_index,
            index_of_max,
            epsilon_square,
            outfile,
        );

        recursively_simplify(
            dt,
            start_time,
            curves,
            curve_index,
            index_of_max,
            fragment_end_index,
            epsilon_square,
            outfile,
        );
    } else {
        // write_row(outfile, &curves[fragment_start_index]);
        writeln!(outfile, "{:.6}\t{:.6}", a[0], a[1]).unwrap();
        // println!("Save [{}]", fragment_start_index);
        // write!(outfile, "{}\t{}\n", &a[0], &a[1]).unwrap();
    }
}

#[allow(dead_code)]
pub fn distance_point_to_line_squared_2d(
    first_point: [f64; 2],
    last_point: [f64; 2],
    mid_point: [f64; 2],
) -> f64 {
    // if first_point.len() != last_point.len() || first_point.len() != mid_point.len() {
    //     panic!("vectors lengths are not equal!");
    // }
    ((last_point[0] - first_point[0]) * (last_point[1] - mid_point[1])
        - (last_point[1] - first_point[1]) * (first_point[0] - mid_point[0]))
        .abs()
        / ((last_point[0] - first_point[0]).powi(2) + (last_point[1] - first_point[1]).powi(2))
            .sqrt()
}
