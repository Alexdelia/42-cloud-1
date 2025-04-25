use super::relative::relative;

pub fn gradient(accuracy: f64) -> String {
	let min = 80.0;
	let max = 100.0;
	let hue = relative(accuracy, min, max, 0.0, 120.0);
	let sat = relative(accuracy, min, max, 40.0, 70.0);
	format!("color: hsl({hue}, {sat}%, 50%)")
}
