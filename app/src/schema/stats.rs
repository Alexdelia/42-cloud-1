use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct Stats {
	start_time: DateTime<Utc>,
	end_time: DateTime<Utc>,

	correct_key: usize,
	wrong_key: usize,
}

impl Default for Stats {
	fn default() -> Self {
		Self {
			start_time: Utc::now(),
			end_time: Utc::now(),

			correct_key: 0,
			wrong_key: 0,
		}
	}
}

impl Stats {
	pub fn wpm(&self) -> f64 {
		let count = self.key_count();
		if count <= 1 {
			return 0.0;
		}

		let duration = self.end_time - self.start_time;
		let minutes = duration.num_milliseconds() as f64 / 1000.0 / 60.0;
		if minutes == 0.0 {
			return 0.0;
		}

		let words = count as f64 / 5.0;
		words / minutes
	}

	pub fn accuracy(&self) -> f64 {
		if self.wrong_key == 0 {
			return 100.0;
		}
		if self.correct_key == 0 {
			return 0.0;
		}

		self.correct_key as f64 / self.key_count() as f64 * 100.0
	}

	pub fn key_count(&self) -> usize {
		self.correct_key + self.wrong_key
	}
}
