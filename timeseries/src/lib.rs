mod curve_simplification;
use std::{fs::File, io::BufWriter};

pub enum Segment {
    Single(Vec<f64>),
    Multiple(Vec<Vec<f64>>),
}

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
    segment: Segment,
    output_files: Vec<BufWriter<File>>,
}

impl Timeseries {
    pub fn new(
        dt: f64,
        num_nodes: usize,
        dim_save_state: usize,
        segment_size: usize,
        dyn_var_names: Vec<String>,
    ) -> Self {
        let full_dimension = num_nodes * dim_save_state;

        // let output_files = ;

        Timeseries {
            dt,
            segment_start_time: 0.0,
            segment: match full_dimension {
                0 => panic!("cannot create segment with 0 data."),
                1 => Segment::Single(vec![0.0; segment_size]),
                _ => Segment::Multiple(vec![vec![0.0; full_dimension]; segment_size]),
            },
            output_files: (0..num_nodes)
                .into_iter()
                .map(|node| {
                    (0..dim_save_state)
                        .into_iter()
                        .map(|var_name| {
                            let file_name =
                                format!("curve_{:02}_{}.txt", &node, &dyn_var_names[var_name]);
                            let mut file = File::create(&file_name).unwrap();
                            BufWriter::new(file)
                        })
                        .collect::<Vec<BufWriter<File>>>()
                })
                .flatten()
                .collect::<Vec<BufWriter<File>>>(),
        }
    }

    pub fn segment(&mut self) -> &mut Segment {
        &mut self.segment
    }

    // save simplified timeseries in individual files
    pub fn save_simplified_timeseries(&mut self) {}

    // save simplified parametric plots
    pub fn save_simplified_parametric_curves(&mut self) {}

    // save subsets of timeseries
    pub fn save_simplified_timeseries_subsets(&mut self, keep_indices: &[usize]) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    // for example
    #[test]
    fn it_works() {
        let a = 0;
        assert_eq!(0, a);
    }
}
