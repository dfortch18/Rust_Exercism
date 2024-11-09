#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s }
    }
}

pub trait Planet {
    fn years_during(&self, d: &Duration) -> f64;
    fn name(&self) -> &'static str;
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
    fn years_during(&self, d: &Duration) -> f64 {
        Earth.years_during(d) / 0.2408467
    }
    fn name(&self) -> &'static str {
        "Mercury"
    }
}

impl Planet for Venus {
    fn years_during(&self, d: &Duration) -> f64 {
        Earth.years_during(d) / 0.61519726
    }
    fn name(&self) -> &'static str {
        "Venus"
    }
}

impl Planet for Earth {
    fn years_during(&self, d: &Duration) -> f64 {
        d.seconds as f64 / 31557600.0
    }
    fn name(&self) -> &'static str {
        "Earth"
    }
}

impl Planet for Mars {
    fn years_during(&self, d: &Duration) -> f64 {
        Earth.years_during(d) / 1.8808158
    }
    fn name(&self) -> &'static str {
        "Mars"
    }
}

impl Planet for Jupiter {
    fn years_during(&self, d: &Duration) -> f64 {
        Earth.years_during(d) / 11.862615
    }
    fn name(&self) -> &'static str {
        "Jupiter"
    }
}

impl Planet for Saturn {
    fn years_during(&self, d: &Duration) -> f64 {
        Earth.years_during(d) / 29.447498
    }
    fn name(&self) -> &'static str {
        "Saturn"
    }
}

impl Planet for Uranus {
    fn years_during(&self, d: &Duration) -> f64 {
        Earth.years_during(d) / 84.016846
    }
    fn name(&self) -> &'static str {
        "Uranus"
    }
}

impl Planet for Neptune {
    fn years_during(&self, d: &Duration) -> f64 {
        Earth.years_during(d) / 164.79132
    }
    fn name(&self) -> &'static str {
        "Neptune"
    }
}
