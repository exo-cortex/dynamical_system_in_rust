use std::fs::File;
use std::io::{BufWriter, Write};

#[allow(dead_code)]
pub fn simplify_curves_individually(
    dt: f64,
    start_time: f64,
    curves: &Vec<Vec<f64>>,
    epsilon: &f64,
    outfiles: &mut [BufWriter<File>],
    nums_written_lines: &mut Vec<u64>,
) {
    for ((i, outfile), num_lines) in &mut outfiles.iter_mut().enumerate().zip(nums_written_lines) {
        write!(outfile, "{}\t{}\n", start_time as f64, &curves[0][i]).unwrap();
        recursively_simplify_with_time(
            dt,
            start_time,
            curves,
            i,
            0,
            curves.len() - 1,
            epsilon.powi(2),
            outfile,
            num_lines,
            // 0,
        );
        // write_row(outfile, (curve.last().unwrap()));
        // outfile.flush().unwrap();
        // println!();
    }
}

#[allow(dead_code)]
fn recursively_simplify_with_time(
    dt: f64,
    start_time: f64,
    curves: &Vec<Vec<f64>>,
    curve_index: usize,
    first_element: usize,
    last_element: usize,
    epsilon_square: f64,
    outfile: &mut BufWriter<File>,
    num_lines: &mut u64,
) {
    let mut max_square_distance = 0.0;
    let mut index_of_max = first_element + 1;

    let a = [
        start_time + dt * first_element as f64,
        curves[first_element][curve_index],
    ];
    let b = [
        start_time + dt * last_element as f64,
        curves[last_element][curve_index],
    ];

    // for i in first_element + 1..last_element - 1 {
    for (i, row) in curves
        .iter()
        .enumerate()
        // until `last_element - 1`
        .take(last_element - 1)
        // start at `first_element + 1`
        .skip(first_element + 1)
    {
        // print!("{i} ");
        // let mid_point = [start_time + dt * i as f64, curves[i][curve_index]];
        let mid_point = [start_time + dt * i as f64, row[curve_index]];

        let square_distance = distance_point_to_line_squared_2d(a, b, mid_point);
        if max_square_distance < square_distance {
            max_square_distance = square_distance;
            index_of_max = i;
        }
    }

    if max_square_distance > epsilon_square {
        recursively_simplify_with_time(
            dt,
            start_time,
            curves,
            curve_index,
            first_element,
            index_of_max,
            epsilon_square,
            outfile,
            num_lines,
        );
        recursively_simplify_with_time(
            dt,
            start_time,
            curves,
            curve_index,
            index_of_max,
            last_element,
            epsilon_square,
            outfile,
            num_lines,
        );
    } else {
        writeln!(
            outfile,
            "{:.6}\t{:.6}",
            start_time + dt * index_of_max as f64,
            curves[index_of_max][curve_index]
        )
        .unwrap();
        *num_lines += 1;
    }
}

#[allow(dead_code)]
pub fn simplify_parametric_subset_curve(
    curves: &Vec<Vec<f64>>,
    index_1: usize,
    index_2: usize,
    epsilon: f64,
    outfile: &mut BufWriter<File>,
    nums_written_lines: &u64,
) {
    write!(
        outfile,
        "{}\t{}\n",
        &curves[0][index_1], &curves[0][index_2]
    )
    .unwrap();
    recursively_simplify_subset_pair(
        curves,
        index_1,
        index_2,
        0,
        curves.len() - 1,
        epsilon.powi(2),
        outfile,
    );
    // write_row(outfile, (curve.last().unwrap()));
    // outfile.flush().unwrap();
    // println!();
}

#[allow(dead_code)]
fn recursively_simplify_subset_pair(
    curves: &Vec<Vec<f64>>,
    index_1: usize,
    index_2: usize,
    first_element: usize,
    last_element: usize,
    epsilon_square: f64,
    outfile: &mut BufWriter<File>,
) {
    let mut max_square_distance = 0.0;
    let mut index_of_max = first_element + 1;

    let a = [
        curves[first_element][index_1],
        curves[first_element][index_2],
    ];
    let b = [curves[last_element][index_1], curves[last_element][index_2]];

    // for i in first_element + 1..last_element - 1 {
    for (i, row) in curves
        .iter()
        .enumerate()
        // until `last_element - 1`
        .take(last_element - 1)
        // start at `first_element + 1`
        .skip(first_element + 1)
    {
        // print!("{i} ");
        // let mid_point = [start_time + dt * i as f64, curves[i][curve_index]];
        let mid_point = [row[index_1], row[index_2]];

        let square_distance = distance_point_to_line_squared_2d(a, b, mid_point);
        if max_square_distance < square_distance {
            max_square_distance = square_distance;
            index_of_max = i;
        }
    }

    if max_square_distance > epsilon_square {
        recursively_simplify_subset_pair(
            curves,
            index_1,
            index_2,
            first_element,
            index_of_max,
            epsilon_square,
            outfile,
        );
        recursively_simplify_subset_pair(
            curves,
            index_1,
            index_2,
            index_of_max,
            last_element,
            epsilon_square,
            outfile,
        );
    } else {
        writeln!(
            outfile,
            "{:.6}\t{:.6}",
            curves[index_of_max][index_1], curves[index_of_max][index_2]
        )
        .unwrap();
    }
}

#[allow(dead_code)]
pub fn distance_point_to_line_2d(a: [f64; 2], b: [f64; 2], p: [f64; 2]) -> f64 {
    // if a.len() != b.len() || a.len() != p.len() {
    //     panic!("vectors lengths are not equal!");
    // }
    ((b[0] - a[0]) * (a[1] - p[1]) - (b[1] - a[1]) * (a[0] - p[0])).abs()
        / ((b[0] - a[0]).powi(2) + (b[1] - a[1]).powi(2)).sqrt()
}

#[allow(dead_code)]
pub fn distance_point_to_line_squared_2d(a: [f64; 2], b: [f64; 2], p: [f64; 2]) -> f64 {
    // if a.len() != b.len() || a.len() != p.len() {
    //     panic!("vectors lengths are not equal!");
    // }
    ((b[0] - a[0]) * (a[1] - p[1]) - (b[1] - a[1]) * (a[0] - p[0])).powi(2)
        / ((b[0] - a[0]).powi(2) + (b[1] - a[1]).powi(2))
}
