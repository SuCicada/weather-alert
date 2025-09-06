
pub fn get_day_name(day: i32) -> String {
    match day {
        0 => "today".to_string(),
        1 => "tomorrow".to_string(),
        n if n > 1 => format!("{} days later", n),
        n => format!("day {}", n),
    }
}