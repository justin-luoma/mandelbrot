pub struct Hsv {
    pub hue: f64,
    pub sat: f64,
    pub val: f64,
}

impl Hsv {
    pub fn new(hue: f64, sat: f64, val: f64) -> Self {
        Self {
            hue,
            sat,
            val,
        }
    }
}