mod simplify;

use std::{
    fmt::Display,
    fs::{DirBuilder, File},
    io::BufWriter,
};

/// Describes one output curve: a set of column indices to simplify together.
pub struct OutputCurve {
    pub columns: Vec<usize>,
    pub with_time: bool,
    pub filename: String,
}

/// One file per column: column[i] over time.
pub fn all_timeseries(num_nodes: usize, names: &[&str]) -> Vec<OutputCurve> {
    (0..num_nodes)
        .flat_map(|node| {
            names.iter().enumerate().map(move |(var_idx, name)| {
                let col = node * names.len() + var_idx;
                OutputCurve {
                    columns: vec![col],
                    with_time: true,
                    filename: format!("trajectory_{:02}_{}.txt", node, name),
                }
            })
        })
        .collect()
}

/// Explicit groups of columns (no time).
pub fn parametric_curves(
    num_nodes: usize,
    num_variables: usize,
    groups: &[Vec<usize>],
    names: &[&str],
) -> Vec<OutputCurve> {
    (0..num_nodes)
        .flat_map(|node| {
            groups.iter().map(move |group| {
                let columns: Vec<usize> =
                    group.iter().map(|&v| v + node * num_variables).collect();
                let name_parts: Vec<&str> = group.iter().map(|&v| names[v]).collect();
                OutputCurve {
                    columns,
                    with_time: false,
                    filename: format!(
                        "parametric_curve_{:02}_{}.txt",
                        node,
                        name_parts.join("_")
                    ),
                }
            })
        })
        .collect()
}

struct SaveFile {
    writer: BufWriter<File>,
    written_lines: u64,
}

pub struct SegmentStorage {
    dt: f64,
    segment_start_time: f64,
    segment: Vec<Vec<f64>>,
    output_curves: Vec<OutputCurve>,
    save_files: Vec<SaveFile>,
    written_segments: u64,
}

impl SegmentStorage {
    pub fn new(
        dt: f64,
        total_columns: usize,
        segment_size: usize,
        output_curves: Vec<OutputCurve>,
    ) -> Self {
        let data_directory = "./data";
        DirBuilder::new()
            .recursive(true)
            .create(data_directory)
            .unwrap();

        let save_files = output_curves
            .iter()
            .map(|curve| {
                let path = format!("{}/{}", data_directory, &curve.filename);
                let file = File::create(path).unwrap();
                SaveFile {
                    writer: BufWriter::new(file),
                    written_lines: 0,
                }
            })
            .collect();

        SegmentStorage {
            dt,
            segment_start_time: 0.0,
            segment: vec![vec![0.0; total_columns]; segment_size],
            output_curves,
            save_files,
            written_segments: 0,
        }
    }

    pub fn segment(&mut self) -> &mut Vec<Vec<f64>> {
        &mut self.segment
    }

    pub fn update_time(&mut self, time: &f64) {
        self.segment_start_time = *time;
    }

    pub fn simplify_and_save(&mut self, epsilon: f64) {
        let epsilon_squared = epsilon.powi(2);
        for (curve, file) in self.output_curves.iter().zip(self.save_files.iter_mut()) {
            let time_basis = if curve.with_time {
                Some((self.segment_start_time, self.dt))
            } else {
                None
            };
            simplify::simplify_and_write(
                &self.segment,
                &curve.columns,
                time_basis,
                epsilon_squared,
                &mut file.writer,
                &mut file.written_lines,
            );
        }
        self.written_segments += 1;
    }

    pub fn display_simplification_ratio(&self) {
        let integrated_steps = self.written_segments * self.segment.len() as u64;
        let mut sum_written_lines = 0u64;
        for file in &self.save_files {
            sum_written_lines += file.written_lines;
            println!(
                "compression: {:.1e}",
                file.written_lines as f64 / integrated_steps as f64
            );
        }
        let num_files = self.save_files.len() as u64;
        println!(
            "total compression: {:.1e}",
            sum_written_lines as f64 / (num_files * integrated_steps) as f64
        );
    }
}

impl Display for SegmentStorage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self.segment)
    }
}
