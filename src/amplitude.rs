///
/// 
pub struct Amplitude{
    value: f64,
    array: Vec<i16>,
}
//
//
impl Amplitude{
    ///
    /// Creates new vector of amplitude
    /// 
    fn new(value: f64, len:usize) -> Self{
        Self{
            value: value,
            array: vec![0; len],
        }
    }
    fn calc(&mut self, arrayA: Vec<f64>, arrayK: Vec<f64>, angle: f64) -> f64{
        self.value = 0.0;
        for i in 0..self.array.len() {
            if (i< arrayK.len() && i < arrayA.len()){
                self.value += arrayA[i] * (arrayK[i]*angle).sin();
            }
        }
        self.value
    }
}