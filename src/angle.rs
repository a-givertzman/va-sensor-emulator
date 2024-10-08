use std::f64::consts::PI;

///
/// Struct `Angle`
/// - `freq` - sampling frequency
/// - `angle` - initial angle in radians
/// - `delta` - the difference between the current and previous angle
/// - `lim` - the limit for the angle
pub struct Angle {
    freq: f64,
    angle: f64,
    delta: f64,
    lim: f64,
}
//
//
impl Angle {
    pub const PI2: f64 = 2.0 * PI;
    ///
    /// ....
    /// - `freq` - sampling frequency
    /// - `angle` - initial angle in radians
    pub fn new(freq: usize, phase: f64) -> Self {
        let delta = Self::PI2 / (freq as f64);
        Self {
            freq: freq as f64,
            angle: phase,
            delta,
            lim: Self::PI2 - delta * 0.5
        }
    }
    ///
    /// ..
    pub fn add(&mut self) -> f64 {
        self.angle = (self.angle + self.delta) % self.lim;
        self.angle
    }
}
