// mod curve_simplification;
mod simplify_timeseries;

use std::{
    fs::{DirBuilder, File},
    io::BufWriter,
};

use std::fmt::Display;

pub enum SimplificatonFactor {
    Absolute(f64), // epsilon is absolute
    Relative(f64), // epsilon is relative to current segment's min-max-distance
}

pub enum SaveMethod<'a> {
    All,
    ParametricCurves2d(&'a [usize; 2]),
    ParametricCurveNd(&'a [usize; 2]),
    Single,
    RespectiveSubsets(&'a [usize]), // for each node save a subset
    SingleSubset(&'a [usize]),
}

pub trait ToSlice {
    fn to_slice(&self) -> &[f64];
}

pub struct Timeseries {
    dt: f64,
    segment_start_time: f64,
    segment: Vec<Vec<f64>>,
    output_files: Vec<BufWriter<File>>,
}

impl Timeseries {
    pub fn new(
        dt: f64,
        num_nodes: usize,
        dim_save_state: usize,
        segment_size: usize,
        dyn_var_names: &[&str],
    ) -> Self {
        let full_dimension = num_nodes * dim_save_state;

        assert_eq!(dim_save_state, dyn_var_names.len());

        Timeseries {
            dt,
            segment_start_time: 0.0,
            segment: vec![vec![0.0; full_dimension]; segment_size],
            output_files: Self::setup_files(num_nodes, dyn_var_names),
        }
    }

    fn setup_files(num_nodes: usize, dyn_var_names: &[&str]) -> Vec<BufWriter<File>> {
        let curve_directory_name = "data";
        let data_directory = format!("./{}", &curve_directory_name);
        DirBuilder::new()
            .recursive(true)
            .create(data_directory)
            .unwrap();

        (0..num_nodes)
            .flat_map(|node| {
                dyn_var_names.iter().map(move |var_name| {
                    let filename = format!(
                        "{}/trajectory_{:02}_{}.txt",
                        &curve_directory_name, node, &var_name
                    );
                    let file = File::create(filename).unwrap();
                    BufWriter::new(file)
                })
            })
            .collect::<Vec<BufWriter<File>>>()
    }

    pub fn segment(&mut self) -> &mut Vec<Vec<f64>> {
        &mut self.segment
    }

    pub fn update_time(&mut self, time: &f64) {
        self.segment_start_time = *time;
    }
    // save simplified timeseries in individual files
    pub fn save_simplified_timeseries(&mut self) {
        simplify_timeseries::simplify_curves_individually(
            self.dt,
            self.segment_start_time,
            &self.segment,
            0.001,
            &mut self.output_files,
        );
    }

    // save simplified parametric plots
    // pub fn save_simplified_parametric_curves(&mut self, index_1: usize, index_2: usize) {
    //     simplify_timeseries::simplify_parametric_subset_curve(
    //         &self.segment,
    //         0,
    //         1,
    //         0.001,
    //         &mut self.output_files[0],
    //     )
    // }

    // save subsets of timeseries
    pub fn save_simplified_timeseries_subsets(&mut self, _keep_indices: &[usize]) {}
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
