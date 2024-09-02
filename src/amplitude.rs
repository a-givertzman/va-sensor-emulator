use crate::angle::Angle;

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
    /// Creates `Amplitude` with
    /// - `value` - value of the amplitude
    /// - `len` - length of the buffer
    pub fn new(len:usize) -> Self{
        Self{
            value: 0.,
            array: Vec::new(),
        }
    }
    ///
    /// - Calculates new amplitude
    /// - Returns calculated value
    pub fn calc(&mut self, arrayA: &Vec<f64>, arrayK: &Vec<f64>, angle: f64) -> f64{
        self.value = 0.;
        for i in 0..arrayA.len(){
        //for i in 0..self.array.len() {
            if(i< arrayK.len() && i < arrayA.len()){
                self.value += arrayA[i] * (arrayK[i]+angle).sin();
            }
        }
        self.array.push(self.value as i16);
        //println!("{:?}", self.array);
        self.value
    }
}