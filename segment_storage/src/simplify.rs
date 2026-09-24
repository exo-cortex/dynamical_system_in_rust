use std::fs::File;
use std::io::{BufWriter, Write};

/// Squared perpendicular distance from point to line A→B in N dimensions.
/// Uses projection method: project P onto line, measure residual.
fn distance_squared(a: &[f64], b: &[f64], p: &[f64]) -> f64 {
    let t: f64 = a
        .iter()
        .zip(b)
        .zip(p)
        .map(|((a, b), p)| (p - a) * (b - a))
        .sum::<f64>()
        / a.iter()
            .zip(b)
            .map(|(a, b)| (b - a).powi(2))
            .sum::<f64>();
    a.iter()
        .zip(b)
        .zip(p)
        .map(|((a, b), p)| (p - a - t * (b - a)).powi(2))
        .sum()
}

/// Extract an N-dimensional point from a row, optionally prepending a synthetic time coordinate.
fn extract_point(
    row: &[f64],
    row_index: usize,
    columns: &[usize],
    time_basis: Option<(f64, f64)>,
) -> Vec<f64> {
    let mut point = Vec::with_capacity(columns.len() + usize::from(time_basis.is_some()));
    if let Some((t0, dt)) = time_basis {
        point.push(t0 + dt * row_index as f64);
    }
    for &col in columns {
        point.push(row[col]);
    }
    point
}

/// Write a point as tab-separated values.
fn write_point(outfile: &mut BufWriter<File>, point: &[f64], num_written: &mut u64) {
    let mut first = true;
    for val in point {
        if first {
            first = false;
        } else {
            write!(outfile, "\t").unwrap();
        }
        write!(outfile, "{:.6}", val).unwrap();
    }
    writeln!(outfile).unwrap();
    *num_written += 1;
}

fn recursively_simplify(
    segment: &[Vec<f64>],
    columns: &[usize],
    time_basis: Option<(f64, f64)>,
    first: usize,
    last: usize,
    epsilon_squared: f64,
    outfile: &mut BufWriter<File>,
    num_written: &mut u64,
) {
    if last - first < 2 {
        return;
    }

    let a = extract_point(&segment[first], first, columns, time_basis);
    let b = extract_point(&segment[last], last, columns, time_basis);

    let mut max_dist_sq = 0.0;
    let mut index_of_max = first + 1;

    for i in (first + 1)..last {
        let p = extract_point(&segment[i], i, columns, time_basis);
        let dist_sq = distance_squared(&a, &b, &p);
        if dist_sq > max_dist_sq {
            max_dist_sq = dist_sq;
            index_of_max = i;
        }
    }

    if max_dist_sq > epsilon_squared {
        recursively_simplify(
            segment, columns, time_basis, first, index_of_max, epsilon_squared, outfile,
            num_written,
        );
        recursively_simplify(
            segment, columns, time_basis, index_of_max, last, epsilon_squared, outfile,
            num_written,
        );
    } else {
        let point = extract_point(&segment[index_of_max], index_of_max, columns, time_basis);
        write_point(outfile, &point, num_written);
    }
}

/// Simplify a segment using the Ramer-Douglas-Peucker algorithm and write results.
///
/// - `columns`: which columns from each row form this curve's coordinates
/// - `time_basis`: if `Some((t0, dt))`, a synthetic time coordinate `t0 + dt * i` is prepended
/// - Writes the first point, then RDP-simplified interior points
pub fn simplify_and_write(
    segment: &[Vec<f64>],
    columns: &[usize],
    time_basis: Option<(f64, f64)>,
    epsilon_squared: f64,
    outfile: &mut BufWriter<File>,
    num_written: &mut u64,
) {
    if segment.is_empty() {
        return;
    }

    let first_point = extract_point(&segment[0], 0, columns, time_basis);
    write_point(outfile, &first_point, num_written);

    if segment.len() > 1 {
        recursively_simplify(
            segment,
            columns,
            time_basis,
            0,
            segment.len() - 1,
            epsilon_squared,
            outfile,
            num_written,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_squared_2d() {
        // Point (1,1) to line from (0,0) to (1,0) → distance² = 1
        assert_eq!(
            1.0,
            distance_squared(&[0.0, 0.0], &[1.0, 0.0], &[1.0, 1.0])
        );

        // Point (1,1) to line from (0,1) to (1,0) → distance² = 0.5
        let d = distance_squared(&[0.0, 1.0], &[1.0, 0.0], &[1.0, 1.0]);
        assert_eq!(0.5_f32, d as f32);
    }

    #[test]
    fn test_distance_squared_3d() {
        // Point (0,0,1) to line from (0,0,0) to (1,0,0) → distance² = 1
        assert_eq!(
            1.0,
            distance_squared(&[0.0, 0.0, 0.0], &[1.0, 0.0, 0.0], &[0.0, 0.0, 1.0])
        );
    }
}
