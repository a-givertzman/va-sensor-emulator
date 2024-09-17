pub struct Complex{
    re: f64,
    im: f64,
}
//
//
impl Complex{
    ///
    /// Creates complex number with
    /// - `re` - real part
    /// - `im` - imaginary part
    pub fn new(re: f64, im: f64) -> Self{
        Self { 
            re, 
            im, 
        }
    }
    ///
    /// - Returs argument of the complex number
    pub fn arg(&mut self) -> f64{
        self.im.atan2(self.re)
    }
}