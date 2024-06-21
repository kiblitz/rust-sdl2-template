use std::time::Duration;

const NS_IN_S: u32 = 1_000_000_000u32;
const REFRESH_RATE: u32 = 60;
pub const REFRESH_EVERY: Duration = Duration::new(0, NS_IN_S / REFRESH_RATE);

#[warn(dead_code)]
pub fn lerp(current: f32, target: f32, c: f32, delta_time: f32) -> f32 {
    let percent = 1. - c.powf(delta_time);
    return current + (target - current) * percent;
}
