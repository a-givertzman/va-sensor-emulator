///
/// ...
pub struct Buffer {
    array: Vec<i16>,
    index: usize,
}
//
//
impl Buffer{
    ///
    /// Creats `Buffer` with
    /// - `len` - length of the buffer
    pub fn new(len: usize) -> Buffer{
        Buffer{
            array: Vec::with_capacity(len),
            index: len,
        }
    }
    ///
    /// - Adds new angle value to the buffer
    /// - Returns buffer if if's full
    pub fn add(&mut self, angle: f64) -> Option<Vec<i16>>{
        match self.array.len() < self.index {
            true => {
                self.array.push(angle.round() as i16);
                None
            }
            false => {
                Some(self.array.clone())
            }
        }

    }
}
