#[cfg(test)]
//сделать класс и формулы для комплексного числа
mod amplitude {
    use std::{array, borrow::Borrow, f64::consts::PI, result, sync::Once, time::{Duration, Instant}};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};

    use crate::{amplitude::{self, Amplitude}, angle::Angle, stuff::approx_eq::AproxEq, complex::Complex};
    ///
    ///
    static INIT: Once = Once::new();
    ///
    /// once called initialisation
    fn init_once() {
        INIT.call_once(|| {
            // implement your initialisation code to be called only once for current test file
        })
    }
    ///
    /// returns:
    ///  - ...
    fn init_each() -> () {}
    ///
    /// Calculating amplitude on static arrays
    fn formula(alpha: f64, array_a:&Vec<f64>, array_phi: &Vec<f64>) -> f64{
        log::debug!("alpha: {}", alpha);
        array_a[0]*(alpha + array_phi[0]).sin() + array_a[1]*(alpha + array_phi[1]).sin() + array_a[2]*(alpha + array_phi[2]).sin()
    }
    ///
    /// Testing amplitude on `exact` values
    #[test]
    fn exact_values() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let self_id = "test";
        log::debug!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        //do not change capacity during test
        let array_a = vec![1., 2., 3.];
        let array_phi = vec![0.1, 0.2, 0.3];

        log::debug!("array_a: {:?} \narray_phi: {:?}", array_a, array_phi);
            
        //manually calculated values
        let test_data = [
            (90.*PI/180., 5.8211), //(angle in radians, calculated amplitude)
            (234.*PI/180., 0.),
            (180.*PI/180., -1.3837),
            (270.*PI/180., -5.8211),
            (360.*PI/180., 1.3837),
            (0.*PI/180., 1.3837),
            (45.*PI/180., 5.0946),
            (30.*PI/180., 4.1089),
        ];

        //the values calculated by static formula
        // let test_data = [
        //     (90.*PI/180., formula(90.*PI/180., &array_a, &array_phi)), //(angle in radians, calculated amplitude)
        //     (180.*PI/180., formula(180.*PI/180., &array_a, &array_phi)),
        //     (270.*PI/180., formula(270.*PI/180., &array_a, &array_phi)),
        //     (360.*PI/180., formula(360.*PI/180., &array_a, &array_phi)),
        //     (0.*PI/180., formula(0.*PI/180., &array_a, &array_phi)),
        //     (45.*PI/180., formula(45.*PI/180., &array_a, &array_phi)),
        //     (30.*PI/180., formula(30.*PI/180., &array_a, &array_phi)),
        // ];
        
        for (angle, target) in test_data {
            log::debug!("amplitude: {}", target);
            let mut amplitude = Amplitude::new(array_a.clone(), array_phi.clone()); //(array_a/k.len());
            let mut result = amplitude.calc(angle);
            log::debug!("angle {} \nresult: {:?}\ntarget: {:?}", angle *180./PI, result, target);
            assert!(result.aprox_eq(target, 3));
        }
        test_duration.exit();
    }
    
    fn cycle_formula(alpha: f64, array_a:&Vec<f64>, array_phi: &Vec<f64>) -> f64{
        let mut value = 0.;
        for i in 0..array_a.len(){
            value += array_a[i]*(alpha + array_phi[i]).sin();
        }
        value
    }
    ///
    /// Testing amplitude on `random` values
    #[test]
    fn random_values(){
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let self_id = "test";
        log::debug!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();
        use rand::{Rng, thread_rng};

        let array_capacity = thread_rng().gen_range(1..=100);
        let mut array_a = vec![0.; array_capacity];
        let mut array_phi = vec![0.; array_capacity];

        for i in 0..array_capacity{
            array_a.push(thread_rng().gen_range(0.0..=1000.0));
            array_phi.push(thread_rng().gen_range(0.0..=1000.0));
        }

        let data_capacity = thread_rng().gen_range(1..=100);
        let mut test_data :Vec<(f64, f64)>= Vec::new();
        let mut rand_angle = 0.;
        let mut calc_amplitude = 0.;

        let mut num_type = false; //true - Real, false - Complex

        for i in 0..data_capacity{
            num_type = thread_rng().gen();
            log::debug!("num_type: {}", num_type);

            if(num_type){
                rand_angle = thread_rng().gen_range(0.0..=360.0) * PI/180.;
                log::debug!("real angle in rad: {}", rand_angle);
            }
            else{
                let re = thread_rng().gen_range(0.0..=100.0);
                let im = thread_rng().gen_range(0.0..=100.0);
                let mut complex = Complex::new(re, im);
                log::debug!("complex number: {} + {}i", re, im);
                rand_angle = complex.arg();
                log::debug!("complex angle in rad: {}", rand_angle);
                log::debug!("complex angle in grad: {}", rand_angle * 180./PI);
            }
            calc_amplitude  = cycle_formula(rand_angle, &array_a, &array_phi);

            test_data.push((rand_angle, calc_amplitude));
        }

        for (angle, target) in test_data {
            log::debug!("amplitude: {}", target);
            let mut amplitude = Amplitude::new(array_a.clone(), array_phi.clone());
            let mut result = amplitude.calc(angle);
            log::debug!("angle {} \nresult: {:?}\ntarget: {:?}", angle *180./PI, result, target);
            assert!(result.aprox_eq(target, 3));
        }
        test_duration.exit();
    }
}