use std::time::Duration;

const NS_IN_S: u32 = 1_000_000_000u32;
const REFRESH_RATE: u32 = 60;
pub const REFRESH_EVERY: Duration = Duration::new(0, NS_IN_S / REFRESH_RATE);

#[inline(always)]
pub fn lerp(current: f32, target: f32, c: f32, delta_time: f32) -> f32 {
    let percent = 1. - c.powf(delta_time);
    current + (target - current) * percent
}

#[inline(always)]
pub fn drag(current: f32, c: f32, delta_time: f32) -> f32 {
    current * c.powf(delta_time)
}
