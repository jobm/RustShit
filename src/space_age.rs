use std::collections::HashMap;

const EARTH_YEAR: f64 = 365.25;
const EARTH_SECS: f64 = 86400.00;

#[derive(Debug)]
pub struct Duration(u64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s)
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        calc_age("Mercury", d.0)
    }
}

impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        calc_age("Venus", d.0)
    }
}

impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        calc_age("Earth", d.0)
    }
}

impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        calc_age("Mars", d.0)
    }
}
impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        calc_age("Jupiter", d.0)
    }
}
impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        calc_age("Saturn", d.0)
    }
}
impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        calc_age("Uranus", d.0)
    }
}
impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        calc_age("Neptune", d.0)
    }
}

// Helper functions
pub fn calc_age(planet: &str, duration: u64) -> f64 {
    let map = HashMap::<&str, f64>::from([
        ("Mercury", 0.2408467),
        ("Venus", 0.61519726),
        ("Earth", 1.0),
        ("Mars", 1.8808158),
        ("Jupiter", 11.862615),
        ("Saturn", 29.447498),
        ("Uranus", 84.016846),
        ("Neptune", 164.79132),
    ]);
    let period = *map.get(&planet).unwrap();
    format!(
        "{:.2}",
        ((duration as f64) / ((EARTH_SECS * period) * (EARTH_YEAR * period))) * period
    )
    .parse()
    .unwrap()
}
// let seconds = 1000000000;
// let duration = space_age::Duration::from(seconds);
// let output = space_age::Earth::years_during(&duration);
// let expected = 31.69;
// println!("({expected}) Earth Age -> {}", output);
// // assert_eq!(output, expected);

// let seconds = 2134835688;
// let duration = Duration::from(seconds);
// let output = Mercury::years_during(&duration);
// let expected = 280.88;
// println!("({expected}) Mercury Age -> {}", output);
