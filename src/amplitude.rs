pub struct Amplitude{
    value: f64,
    array_a: Vec<f64>,
    array_phi: Vec<f64>,
}
//
//
impl Amplitude{
    ///
    /// Creates `Amplitude` with
    /// - `value` - value of the amplitude
    /// - `len` - length of the buffer
    pub fn new(array_a: Vec<f64>, array_phi: Vec<f64>) -> Self{
        Self{
            value: 0.,
            array_a,
            array_phi,
        }
    }
    ///
    /// - Calculates new amplitude
    /// - Returns calculated value
    pub fn calc(&mut self, angle: f64) -> f64{
        self.value = 0.;
        log::debug!("angle: {}", angle);
        for i in 0..self.array_a.len(){
            self.value += self.array_a[i]*(angle + self.array_phi[i]).sin();
        }
        self.value
    }

}