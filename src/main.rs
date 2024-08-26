//use std::f64::consts::PI;
use half::f16;
use std::f32::consts::PI;
use std::ptr::addr_of_mut;

/*struct Angle{
    fs: f16,
    delta_alpha: f16,
    alpha: f16
}
impl Angle{
    fn new() -> Angle{
        //let delta_alpha = 2.0*PI/self.fs;
        Angle{
            fs: f16::from_f64(300000.),
            delta_alpha: f16::from_f64(0.),
            alpha: f16::from_f64(0.),
        }
    }

    fn add(&mut self) -> f16{
        let pi = f16::from_f64(std::f64::consts::PI);
        println!("{}", pi);
        println!("{}", f16::from_f64(2.0) * pi);
        println!("{}", self.fs);
        self.delta_alpha = f16::from_f64(2.0) * pi/self.fs;
        self.alpha += self.delta_alpha;
        if(self.alpha > f16::from_f64(2.0)* pi){
            self.alpha -= f16::from_f64(2.0) * pi;
        }
        self.alpha

    }
}*/
struct Angle{
    fs: f32,
    delta_alpha: f32,
    alpha: f32
}
impl Angle{
    fn new() -> Angle{
        //let delta_alpha = 2.0*PI/self.fs;
        Angle{
            fs: 300000.,
            delta_alpha: 0.,
            alpha: 0.
        }
    }

    fn add(&mut self) -> f32{
        self.delta_alpha = 2. * PI/self.fs;
        self.alpha += self.delta_alpha;
        if(self.alpha > 2. * PI){
            self.alpha -= 2. * PI;
        }
        self.alpha
    }
}

struct Buffer{
    array: Vec<f32>,
    length: usize
}
impl Buffer{
    fn new(length: usize) -> Buffer{
        Buffer{
            array: Vec::with_capacity(length),
            length
        }
    }

    fn add(&mut self, value: f32) -> Option<Vec<f32>>{
        match self.array.len() < self.length{
            true => {
                self.array.push(value);
                None
            }
            false => {
                Some(self.array.clone())
            }
        }

    }
}
fn main() {
    let mut angle = Angle::new();

    let mut buf = Buffer::new(3);
    for value in 0..10{ // must be endless loop
        let new_angle = angle.alpha;
        match buf.add(new_angle){
            None => println!("Added value is {}", new_angle),
            Some(_) => {
                println!("Created array is {:?}", buf.array);
                buf.array.clear();
            }
        }
        angle.add();
    }
}
