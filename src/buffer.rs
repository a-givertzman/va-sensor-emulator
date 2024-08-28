///
/// Struct 'Buffer'
/// - 'array' -  vector for buffers values
/// - 'index' - length of the buffer
pub struct Buffer {
    array: Vec<i16>,
    index: usize,
    len: usize,
}
//
//
impl Buffer{
    ///
    /// Creats `Buffer` with
    /// - `len` - length of the buffer
    pub fn new(len: usize) -> Buffer{
        Buffer{
            array: vec![0; len],
            index: 0,
            len,
        }
    }
    ///
    /// - Adds new angle value to the buffer and returns "None"
    /// - Returns buffer it self if it's full
    pub fn add(&mut self, angle: f64) -> Option<Vec<i16>>{
        self.array[self.index] = angle.round() as i16;
        let result = if self.index < (self.len - 1) {
            None
        } else {
            Some(self.array.clone())
        };
        self.index = (self.index + 1) % self.len;
        result
    }
}
