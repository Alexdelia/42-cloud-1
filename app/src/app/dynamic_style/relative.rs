pub fn relative(k: f64, min: f64, max: f64, low: f64, high: f64) -> f64 {
	(low + (high - low) * ((k - min) / (max - min)))
		.max(low)
		.min(high)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn relative_test() {
		assert_eq!(relative(0.0, 0.0, 1.0, 0.0, 1.0), 0.0);
		assert_eq!(relative(0.5, 0.0, 1.0, 0.0, 1.0), 0.5);
		assert_eq!(relative(1.0, 0.0, 1.0, 0.0, 1.0), 1.0);
		assert_eq!(relative(0.0, 0.8, 1.0, 42.0, 84.0), 42.0);
		assert_eq!(relative(0.5, 0.8, 1.0, 42.0, 84.0), 42.0);
		assert_eq!(relative(0.9, 0.8, 1.0, 42.0, 84.0), (42.0 + 84.0) / 2.0);
		assert_eq!(relative(1.0, 0.8, 1.0, 42.0, 84.0), 84.0);

		assert_eq!(relative(80.0, 80.0, 100.0, 0.0, 120.0), 0.0);
		assert_eq!(relative(90.0, 80.0, 100.0, 0.0, 120.0), 60.0);
		assert_eq!(relative(100.0, 80.0, 100.0, 0.0, 120.0), 120.0);

		assert_eq!(relative(80.0, 80.0, 100.0, 40.0, 70.0), 40.0);
		assert_eq!(relative(90.0, 80.0, 100.0, 40.0, 70.0), 55.0);
		assert_eq!(relative(100.0, 80.0, 100.0, 40.0, 70.0), 70.0);
	}
}
