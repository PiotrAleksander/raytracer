use std::cell::RefCell;

use rand::rngs::SmallRng;
use rand::RngExt;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}

thread_local! {
    static RNG: RefCell<SmallRng> = RefCell::new(rand::make_rng());
}

pub fn random_f64() -> f64 {
    RNG.with(|rng| rng.borrow_mut().random())
}

pub fn random_range(min: f64, max: f64) -> f64 {
    RNG.with(|rng| rng.borrow_mut().random_range(min..max))
}
