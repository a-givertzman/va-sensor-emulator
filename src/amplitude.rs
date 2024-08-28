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
    fn new() -> Self{
        Self{
            value,
            array,
        }
    }
    fn calc(&mut self, arrayA: Vec<i16>, arrayK: Vec<i16>, angle: f64) -> f64{
        self.value = 0.0;
        for i in 0..self.array.len() {
            if (i< arrayK.len() && i < arrayA.len()){
                self.value += arrayA[i] * (arrayK[i]*angle).sin();
            }
        }
        self.value
    }
}