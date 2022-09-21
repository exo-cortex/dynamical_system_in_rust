// todo:
// -find extrema
// -count extrema

#[allow(dead_code)]
pub struct DynamicalSystemAnalysis<'a> {
	curve_segment: &'a [f64],
	time: f64,
	step_size: f64,
	size: usize,
	average: f64,
	min: Vec<(f64, f64)>,
	max: Vec<(f64, f64)>,
	doublecount_tolerance: f64,
	uniq_min: Vec<f64>,
	uniq_max: Vec<f64>,
	two_before: f64,
	one_before: f64,
}

impl<'a> DynamicalSystemAnalysis<'a> {
	pub fn new(segment: &'a [f64], time: f64, step_size: f64) -> Self {
		DynamicalSystemAnalysis {
			curve_segment: segment,
			time: time,
			step_size: step_size,
			size: 0,
			average: 0.0,
			min: Vec::new(),
			max: Vec::new(),
			doublecount_tolerance: 10.0,
			uniq_min: Vec::new(),
			uniq_max: Vec::new(),
			two_before: 0.0,
			one_before: 0.0,
		}
	}

	pub fn analyse(&mut self) {
		// [(time - 2 step_size, 2bf), (time - 1 step_size, 1bf), (time, now)]
		self.check(
			self.two_before,
			self.one_before,
			self.curve_segment[0],
			self.time - self.step_size,
		);
		self.check(
			self.one_before,
			self.curve_segment[0],
			self.curve_segment[1],
			self.time,
		);
		// check for rest of segment
		let mut segment_sum: f64 = self.curve_segment[0] + self.curve_segment[1];
		self.time += self.step_size;
		println!("{}", self.curve_segment.len());
		for w in self.curve_segment.windows(3) {
			self.check(w[0], w[1], w[2], self.time);
			self.time += self.step_size;
			segment_sum += w[2];
		}
		self.average = (segment_sum + self.average * self.size as f64)
			/ ((self.size + self.curve_segment.len()) as f64);
		self.size += self.size;
		self.two_before = self.curve_segment[self.curve_segment.len() - 2];
		self.one_before = self.curve_segment[self.curve_segment.len() - 1];
		println!(
			"sum: {}, time: {}, average: {}",
			segment_sum, self.time, self.average
		);
	}

	fn check(&mut self, left: f64, middle: f64, right: f64, time: f64) {
		// might be made more rusty
		if left < middle && middle > right {
			self.max.push((time, middle));
			if self.uniq_max.is_empty() {
				self.uniq_max.push(middle);
			} else {
				// this is shitty!
				for max_i in 0..self.uniq_max.len() {
					if (middle - self.uniq_max[max_i]).abs() < self.doublecount_tolerance {
						self.uniq_max.push(middle);
						println!("unique maximum!");
					}
				}
			}
			println!("maximum");
		} else if left > middle && middle < right {
			self.min.push((time, middle));
			if self.uniq_min.is_empty() {
			} else {
				for min_i in 0..self.uniq_min.len() {
					if (middle - self.uniq_min[min_i]).abs() < self.doublecount_tolerance {
						self.uniq_min.push(middle);
						println!("unique minimum!");
					}
				}
			}
			println!("minimum");
		} else {
		}
	}
}
