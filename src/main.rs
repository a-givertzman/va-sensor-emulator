struct Buffer{
    array: Vec<i16>,
    length: usize
}
impl Buffer{
    fn new(length: usize) -> Buffer{
        Buffer{
            array: Vec::with_capacity(length),
            length
        }
    }

    fn add(&mut self, angle: f64) -> Option<Vec<i16>>{
        match self.array.len() < self.length{
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
fn main() {

}
