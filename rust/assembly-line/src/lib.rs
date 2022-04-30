const CARS_PER_HOUR: u32 = 221;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    CARS_PER_HOUR as f64  * match speed {
        0 => 0.0,
        1..=4 => speed as f64,
        5..=8 => 0.90 * speed as f64,
        9..=10 => 0.77 * speed as f64,
        _ => panic!("Not a valid speed"),
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    production_rate_per_hour(speed) as u32 / 60u32
}
