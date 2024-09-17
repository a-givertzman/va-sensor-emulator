///
/// Amplitude ....
/// 
/// Conf Example:
/// ```yaml
///     # A:   ϕ, rad
///      10.1: 0.0
///      07.7: 0.0
///     120.0: 0.0
/// ```
pub struct Amplitude {
    /// `Vec<(A, ϕ)>`
    params: Vec<(f64, f64)>,
}
//
//
impl Amplitude {
    ///
    /// Creates `Amplitude` with
    /// - `conf` - yaml value containing a configuration
    ///
    /// ```yaml
    ///     # A:   ϕ, rad
    ///      10.1: 0.0
    ///      07.7: 0.0
    ///     120.0: 0.0
    /// ```
    pub fn new(params: Vec<(f64, f64)>) -> Self {
        Self {
            params,
        }
    }
    ///
    /// - Calculates new amplitude
    /// - Returns calculated value
    pub fn calc(&self, angle: f64) -> f64 {
        log::debug!("angle: {}", angle);
        self.params.iter().fold(0.0, |value, (amp, phi)| {
            value + *amp * (angle + *phi).sin()
        })
    }

}