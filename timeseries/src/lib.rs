mod simplify_timeseries;

use std::{
    fs::{DirBuilder, File},
    io::BufWriter,
};

use std::fmt::Display;

#[derive(Clone)]
pub enum SaveItems {
    Timeseries,
    ParametricCurve2d { variable_pairs: Vec<[usize; 2]> },
    TimeseriesAndParametricCurve2d { variable_pairs: Vec<[usize; 2]> },
}

pub enum SimplificatonFactor {
    Absolute(f64), // epsilon is absolute
    Relative(f64), // epsilon is relative to current segment's min-max-distance
}

struct SaveFiles {
    timeseries_files: Vec<BufWriter<File>>,
    timeseries_written_lines: Vec<u64>,
    parametric_files: Vec<BufWriter<File>>,
    parametric_written_lines: Vec<u64>,
    // maxima_files: Vec<BufWriter<File>>,
}

impl SaveFiles {
    pub fn new(
        save_items: &SaveItems,
        directory_name: &str,
        nodes: usize,
        dyn_variables: &[&str],
    ) -> Self {
        match save_items {
            SaveItems::Timeseries => SaveFiles {
                timeseries_files: Self::setup_timeseries_files(
                    directory_name,
                    nodes,
                    dyn_variables,
                ),
                timeseries_written_lines: vec![0; nodes * dyn_variables.len()],
                parametric_files: Vec::new(),
                parametric_written_lines: Vec::new(),
            },
            SaveItems::ParametricCurve2d { variable_pairs } => SaveFiles {
                timeseries_files: Vec::new(),
                timeseries_written_lines: Vec::new(),
                parametric_files: Self::setup_parametric_curve_files(
                    directory_name,
                    nodes,
                    dyn_variables,
                    variable_pairs,
                ),
                parametric_written_lines: vec![0; nodes * variable_pairs.len()],
            },
            SaveItems::TimeseriesAndParametricCurve2d { variable_pairs } => SaveFiles {
                timeseries_files: Self::setup_timeseries_files(
                    directory_name,
                    nodes,
                    dyn_variables,
                ),
                timeseries_written_lines: vec![0; nodes * dyn_variables.len()],
                parametric_files: Self::setup_parametric_curve_files(
                    directory_name,
                    nodes,
                    dyn_variables,
                    variable_pairs,
                ),
                parametric_written_lines: vec![0; nodes * variable_pairs.len()],
            },
        }
    }

    fn setup_timeseries_files(
        directory_name: &str,
        nodes: usize,
        dyn_variables: &[&str],
    ) -> Vec<BufWriter<File>> {
        (0..nodes)
            .flat_map(|node| {
                dyn_variables.iter().map(move |var_name| {
                    let filename = format!(
                        "{}/trajectory_{:02}_{}.txt",
                        directory_name, node, &var_name
                    );
                    let file = File::create(filename).unwrap();
                    BufWriter::new(file)
                })
            })
            .collect::<Vec<BufWriter<File>>>()
    }

    fn setup_parametric_curve_files(
        directory_name: &str,
        nodes: usize,
        dyn_variables: &[&str],
        variable_pairs: &[[usize; 2]],
    ) -> Vec<BufWriter<File>> {
        (0..nodes)
            .flat_map(|node| {
                variable_pairs.iter().map(move |[first, second]| {
                    let filename = format!(
                        "{}/parametric_curve_{:02}_{}_{}.txt",
                        directory_name, node, dyn_variables[*first], dyn_variables[*second]
                    );
                    let file = File::create(filename).unwrap();
                    BufWriter::new(file)
                })
            })
            .collect::<Vec<BufWriter<File>>>()
    }
}

pub struct Timeseries {
    dt: f64,
    num_nodes: usize,
    num_variables: usize,
    segment_start_time: f64,
    segment: Vec<Vec<f64>>,
    output_files: SaveFiles,
    written_segments: u64,
    // written_lines: Vec<u64>,
}

impl Timeseries {
    pub fn new(
        dt: f64,
        num_nodes: usize,
        dim_save_state: usize,
        segment_size: usize,
        dyn_var_names: &[&str],
        save_items: &SaveItems,
    ) -> Self {
        let full_dimension = num_nodes * dim_save_state;

        assert_eq!(dim_save_state, dyn_var_names.len());

        let curve_directory_name = "data";
        let data_directory = format!("./{}", &curve_directory_name);
        DirBuilder::new()
            .recursive(true)
            .create(&data_directory)
            .unwrap();

        Timeseries {
            dt,
            num_nodes,
            num_variables: dyn_var_names.len(),
            segment_start_time: 0.0,
            segment: vec![vec![0.0; full_dimension]; segment_size],
            output_files: SaveFiles::new(&save_items, &data_directory, num_nodes, dyn_var_names),
            written_segments: 0,
            // written_lines: vec![0; full_dimension],
        }
    }

    pub fn segment(&mut self) -> &mut Vec<Vec<f64>> {
        &mut self.segment
    }

    pub fn update_time(&mut self, time: &f64) {
        self.segment_start_time = *time;
    }

    // save simplified timeseries in individual files
    pub fn save_simplified_timeseries(&mut self, epsilon: &f64) {
        simplify_timeseries::simplify_curves_individually(
            self.dt,
            self.segment_start_time,
            &self.segment,
            epsilon,
            &mut self.output_files.timeseries_files,
            &mut self.output_files.timeseries_written_lines,
        );
        self.written_segments += 1;
    }

    // save simplified parametric plots

    pub fn save_simplified_parametric_curves(
        &mut self,
        variable_pairs: &Vec<[usize; 2]>,
        epsilon: &f64,
    ) {
        // println!(
        //     "saving parametric plots for variable pairs {:?}",
        //     &variable_pairs
        // );
        simplify_timeseries::simplify_parametric_curve_pairs(
            &self.segment,
            self.num_nodes,
            self.num_variables,
            variable_pairs,
            epsilon,
            &mut self.output_files.parametric_files,
            &mut self.output_files.parametric_written_lines,
        );
        self.written_segments += 1;
    }

    // todo: save n-dimensional parametric plots e.g. curve in 3d-space
    // pub fn save_simplified_timeseries_subsets(&mut self, _keep_indices: &[usize]) {}

    // UGLY!!!
    pub fn display_simplification_ratio(&self) {
        let integrated_steps = self.written_segments * self.segment.len() as u64;
        let mut sum_written_lines = 0;
        self.output_files
            .timeseries_written_lines
            .iter()
            .chain(&self.output_files.parametric_written_lines)
            .for_each(|wls| {
                sum_written_lines += *wls;
                println!("compression: {:.1e}", *wls as f64 / integrated_steps as f64);
            });
        println!(
            "total compression: {:.1e}",
            sum_written_lines as f64
                / ((self.output_files.timeseries_files.len()
                    + self.output_files.parametric_files.len())
                    * integrated_steps as usize) as f64
        )
    }
}

impl Display for Timeseries {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self.segment)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use simplify_timeseries::{distance_point_to_line_2d, distance_point_to_line_squared_2d};

    #[test]
    fn test_distance() {
        // distance of point `p` from line through `a` and `b`
        let a_1 = [0.0, 0.0];
        let b_1 = [1.0, 0.0];
        let p_1 = [1.0, 1.0];
        assert_eq!(1.0, distance_point_to_line_2d(a_1, b_1, p_1));

        let a_2 = [0.0, 1.0];
        let b_2 = [1.0, 0.0];
        let p_2 = [1.0, 1.0];
        assert_eq!(
            (0.50_f64).sqrt() as f32,
            distance_point_to_line_2d(a_2, b_2, p_2) as f32
        );

        assert_eq!(
            (0.50_f64) as f32,
            distance_point_to_line_squared_2d(a_2, b_2, p_2) as f32
        );
    }
}
