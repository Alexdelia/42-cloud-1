use super::relative::relative;

pub fn gradient(wpm: f64) -> String {
	if wpm == 0.0 {
		return "color: hsl(360, 50%, 100%)".to_string();
	}

	let min = 30.0;
	let max = 120.0;
	let hue = relative(wpm, min, max, 240.0, 360.0);
	let lum = relative(wpm, min, max, 50.0, 100.0);
	format!("color: hsl({hue}, 50%, {lum}%)")
}
