use std::f64::consts::PI;
struct Angle{
    fs: f64,
    delta_alpha: f64,
    alpha: f64
}
impl Angle{
    fn new() -> Angle{
        Angle{
            fs: 300000.,
            delta_alpha: 0.,
            alpha: 0.
        }
    }

    fn add(&mut self) -> i16{
        self.delta_alpha = 2. * PI/self.fs;
        println!("delta: {}", self.delta_alpha);
        self.alpha += self.delta_alpha;
        println!("alpha {}", self.alpha);
        if(self.alpha > 2. * PI){
            self.alpha -= 2. * PI;
        }
        self.alpha.round() as i16
    }
}

fn main() {}