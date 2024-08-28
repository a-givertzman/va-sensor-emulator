use std::f64::consts::PI;

///
/// 
pub struct Angle {
    freq: usize,
    angle: f64,
}
//
//
impl Angle {
    ///
    /// ....
    /// - `freq` - sampling frequency
    /// - `phase` - initian angle in radians
    pub fn new(freq: usize, phase: f64) -> Self {
        Self {
            freq,
            angle: phase,
        }
    }
    ///
    /// ..
    pub fn add(&mut self) -> f64 {
        let delta = 2.0 * PI;
        self.angle = self.angle + delta;
        self.angle
    }
}
