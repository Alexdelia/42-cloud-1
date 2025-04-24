use const_format::formatcp;

const PRECISION: &str = "REAL";

const KEY_COUNT: &str = r#"(correct_key + wrong_key)"#;
const WORDS: &str = formatcp!("({KEY_COUNT}::{PRECISION} / 5.0)");

const DURATION: &str = r#"(end_time - start_time)"#;
pub const DURATION_SECONDS: &str = formatcp!("ROUND(EXTRACT(EPOCH FROM {DURATION}))::INTEGER");
const DURATION_MINUTE: &str = formatcp!("(EXTRACT(EPOCH FROM {DURATION}) / 60.0)");

pub const WPM: &str = formatcp!("({WORDS} / {DURATION_MINUTE})");

pub const ACCURACY: &str =
	formatcp!("((correct_key::{PRECISION} / {KEY_COUNT}::{PRECISION}) * 100.0)");
