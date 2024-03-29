// extern crate derive_more;
// use derive_more::{Add, Mul}; // AddAssign, MulAssign, Div

use std::fs::File;
use std::io::{BufWriter, Write};

use core::ops::Range;

// do rdp (ramer-douglas-peucker) curve simplification
// assumption: a timeseries has x values in an ascending, equidistant order

// case A: 1 dimensional curve f(t) -> simplify and write (t, f(t)) into file
// case B: n_dimensional curve {f_1(t), ..., f_n(t)} -> simplify and write columns (t, f_n(t)) into file_n respectively

// this shall be used to simplify (and write out) individual timeseries for each dynamic variable
// for f(t) = (x_0(t), x_1(t), ..., x_n(t)) it shall write out
// {{t, x_0},{t, x_1}, ... , {t, x_n}}
#[allow(dead_code)]
pub fn write_n_simplified_curves(
    curve: &Vec<Vec<f64>>,
    epsilon: f64,
    outfiles: &mut [BufWriter<File>],
) {
    for (i, outfile) in &mut outfiles.iter_mut().enumerate().take(curve[0].len() - 1) {
        // write_row(outfile, &curve[0]);
        recursively_simplify_subset_curve(
            curve,
            0,
            i + 1,
            0,
            curve.len() - 1,
            epsilon.powi(2),
            outfile,
        );
        // write_row(outfile, (curve.last().unwrap()));
        outfile.flush().unwrap();
    }
}

#[allow(dead_code)]
fn recursively_simplify_subset_curve(
    curve: &Vec<Vec<f64>>,
    time_index: usize,
    curve_index: usize,
    local_first: usize,
    local_last: usize,
    epsilon_square: f64,
    outfile: &mut BufWriter<File>,
) {
    let mut max_sqr_distance = 0.0;
    let mut index_of_max = local_first + 1;

    // let a: Vec<f64> = indices
    //     .into_iter()
    //     .map(|col| curve[local_first][*col])
    //     .collect::<Vec<f64>>();
    // let b: Vec<f64> = indices
    //     .into_iter()
    //     .map(|col| curve[local_last][*col])
    //     .collect::<Vec<f64>>();

    let a = vec![
        curve[local_first][time_index],
        curve[local_first][curve_index],
    ];
    let b = vec![
        curve[local_last][time_index],
        curve[local_last][curve_index],
    ];
    // println!("a = ({},{}), b = ({},{}) ", a[0], a[1], b[0], b[1]);
    // println!("from {} to {}", local_first, local_last);
    // here cargo clippy makes a suggestion
    // for i in local_first + 1..local_last {
    for (i, row) in curve
        .iter()
        .enumerate()
        .take(local_last)
        .skip(local_first + 1)
    {
        // let point = vec![curve[i][time_index], curve[i][curve_index]];
        // let point = vec![row[time_index], row[curve_index]];
        // println!("p = ({},{})", point[0], point[1]);
        let row_selection = vec![row[time_index], row[curve_index]];
        let sqr_d = distance_point_to_line_squared::<2>(&a, &b, &row_selection);
        if max_sqr_distance < sqr_d {
            max_sqr_distance = sqr_d;
            index_of_max = i;
        }
    }
    if max_sqr_distance > epsilon_square {
        recursively_simplify_subset_curve(
            curve,
            0,
            curve_index,
            local_first,
            index_of_max,
            epsilon_square,
            outfile,
        );
        recursively_simplify_subset_curve(
            curve,
            0,
            curve_index,
            index_of_max,
            local_last,
            epsilon_square,
            outfile,
        );
    } else {
        // write_row(outfile, &curve[local_first]);
        write_row(outfile, &a);
    }
}

// ########################################

// times for each point of the curve are calculated on the fly.
// through `start_time + dt * step`
#[allow(dead_code)]
pub fn ___write_n_simplified_timeseries(
    dt: f64,
    start_time: f64,
    curve: &Vec<Vec<f64>>,
    epsilon: f64,
    outfiles: &mut [BufWriter<File>],
) {
    for (i, outfile) in &mut outfiles.iter_mut().enumerate() {
        // write!(outfile, "{}\t{}\n", start_time as f64, &curve[0][i]).unwrap();
        ___recursively_simplify_subset_curve(
            &dt,
            &start_time,
            &(start_time + dt * (curve.len() - 1) as f64),
            curve,
            i,
            0,
            curve.len() - 1,
            epsilon.powi(2),
            outfile,
        );
        // write_row(outfile, (curve.last().unwrap()));
        outfile.flush().unwrap();
    }
}

#[allow(dead_code)]
fn ___recursively_simplify_subset_curve(
    dt: &f64,
    fragment_start_time: &f64,
    fragment_end_time: &f64,
    curve: &Vec<Vec<f64>>,
    curve_index: usize,

    fragment_start_index: usize,
    fragment_end_index: usize,
    epsilon_square: f64,
    outfile: &mut BufWriter<File>,
) {
    let mut max_sqr_distance = 0.0;
    let mut index_of_max = fragment_start_index + 1;

    let end_time = *fragment_start_time + dt * ((fragment_end_index - fragment_start_index) as f64);

    let a = [
        *fragment_start_time,
        curve[fragment_start_index][curve_index],
    ];
    let b = [*fragment_end_time, curve[fragment_end_index][curve_index]];

    // here cargo clippy makes a suggestion
    // for i in fragment_start_index + 1..fragment_end_index {
    let mut split_time = *fragment_start_time;
    for (i, row) in curve
        .iter()
        .enumerate()
        .take(fragment_end_index)
        .skip(fragment_start_index + 1)
    {
        split_time += *dt;
        // let point = vec![curve[i][time_index], curve[i][curve_index]];
        // let point = vec![row[time_index], row[curve_index]];
        // println!("p = ({},{})", point[0], point[1]);
        let row_selection = [split_time, row[curve_index]];

        let sqr_d = distance_point_to_line_squared_2d(a, b, row_selection);
        if max_sqr_distance < sqr_d {
            max_sqr_distance = sqr_d;
            index_of_max = i;
        }
    }

    println!(
        "from steps: {} to {} / time: {} to {}, max_distance: {} at index {} ",
        fragment_start_index,
        fragment_end_index,
        *fragment_start_time,
        *fragment_end_time,
        max_sqr_distance,
        index_of_max,
    );

    if max_sqr_distance > epsilon_square {
        ___recursively_simplify_subset_curve(
            dt,
            fragment_start_time,
            &split_time,
            curve,
            curve_index,
            fragment_start_index,
            index_of_max,
            epsilon_square,
            outfile,
        );

        ___recursively_simplify_subset_curve(
            dt,
            fragment_start_time,
            &split_time,
            curve,
            curve_index,
            index_of_max,
            fragment_end_index,
            epsilon_square,
            outfile,
        );
    } else {
        // write_row(outfile, &curve[fragment_start_index]);
        // write_row(outfile, &a);

        write!(outfile, "{}\t{}\n", &a[0], &a[1]).unwrap();
    }
}

// ########################################

#[allow(dead_code)]
fn recursive_simplify_subset(
    curve: &Vec<Vec<f64>>,
    curve_indices: &[usize],
    local_first: usize,
    local_last: usize,
    epsilon_square: f64,
    outfile: &mut BufWriter<File>,
) {
    let mut max_sqr_distance = 0.0;
    let mut index_of_max = local_first + 1;

    let a = curve_indices
        .iter()
        .map(|col| curve[local_first][*col])
        .collect();

    let b = curve_indices
        .iter()
        .map(|col| curve[local_last][*col])
        .collect();

    // println!("a = ({},{}), b = ({},{}) ", a[0], a[1], b[0], b[1]);
    // println!("from {} to {}", local_first, local_last);
    // here cargo clippy makes a suggestion
    // for i in local_first + 1..local_last {
    for (i, row) in curve
        .iter()
        .enumerate()
        .take(local_last)
        .skip(local_first + 1)
    {
        // let point = vec![curve[i][time_index], curve[i][curve_index]];
        // let point = vec![row[time_index], row[curve_index]];
        // println!("p = ({},{})", point[0], point[1]);
        let sqr_d = distance_point_to_line_squared_vec(&a, &b, &row);
        if max_sqr_distance < sqr_d {
            max_sqr_distance = sqr_d;
            index_of_max = i;
        }
    }
    if max_sqr_distance > epsilon_square {
        recursive_simplify_subset(
            curve,
            curve_indices,
            local_first,
            index_of_max,
            epsilon_square,
            outfile,
        );
        recursive_simplify_subset(
            curve,
            curve_indices,
            index_of_max,
            local_last,
            epsilon_square,
            outfile,
        );
    } else {
        // write_row(outfile, &curve[local_first]);
        write_row(outfile, &a);
    }
}

#[allow(dead_code)]
pub fn write_simplified_nd_curve(
    curve: &Vec<Vec<f64>>,
    epsilon: f64,
    outfile: &mut BufWriter<File>,
) {
    recursively_simplify_nd_curve(curve, 0, curve.len() - 1, epsilon.powi(2), outfile);
    write_row(outfile, curve.last().unwrap());
}

#[allow(dead_code)]
fn recursively_simplify_nd_curve(
    curve: &[Vec<f64>],
    first: usize,
    last: usize,
    epsilon_square: f64,
    outfile: &mut BufWriter<File>,
) {
    let mut max_sqr_distance = 0.0;
    let mut index_of_max: usize = first + 1;
    for i in first + 1..last {
        let sqr_d = distance_point_to_line_squared_vec(&curve[first], &curve[last], &curve[i]);
        if max_sqr_distance < sqr_d {
            max_sqr_distance = sqr_d;
            index_of_max = i;
        }
    }
    // grandchild proposes this insead:
    // let (index_of_max, max_sqr_distance) = curve
    //     .iter()
    //     .map(|v| distance_point_to_line_squared(&curve[first], &curve[last], v))
    //     .enumerate()
    //     .max_by(|(_, a), (_, b)| a.total_cmp(b))
    //     .unwrap();

    if max_sqr_distance > epsilon_square {
        recursively_simplify_nd_curve(curve, first, index_of_max, epsilon_square, outfile);
        recursively_simplify_nd_curve(curve, index_of_max, last, epsilon_square, outfile);
    } else {
        write_row(outfile, &curve[first]);
    }
}

pub fn write_row(outfile: &mut BufWriter<File>, values: &[f64]) {
    for value in values.iter().take(values.len() - 1) {
        let value_string = format!("{:.10}\t", value);
        write!(outfile, "{}", value_string).unwrap();
    }
    let value_string = format!("{:.10}\n", &values.last().unwrap());
    write!(outfile, "{}", value_string).unwrap();
}

#[allow(dead_code)]
pub fn distance_point_to_line_squared<const N: usize>(a: &[f64], b: &[f64], point: &[f64]) -> f64 {
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

pub fn distance_point_to_line_squared_vec(a: &Vec<f64>, b: &Vec<f64>, point: &Vec<f64>) -> f64 {
    // -> compare to here: https://softwareengineering.stackexchange.com/questions/168572/distance-from-point-to-n-dimensional-line
    // n_vector pa = P - A
    // n_vector ba = B - A
    // double t = dot(pa, ba)/dot(ba, ba)
    // double d = length(pa - t * ba)
    // if a.len() != b.len() || a.len() != point.len() {
    //     panic!("vectors lengths are not equal!");
    // }

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
