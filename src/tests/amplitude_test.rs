#[cfg(test)]

mod amplitude {
    use std::{f64::consts::PI, result, sync::Once, time::{Duration, Instant}};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};

    use crate::{amplitude::{self, Amplitude}, angle::Angle};
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
    /// Testing such functionality / behavior
    #[test]
    fn check_ampl_value() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let self_id = "test";
        log::debug!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();
        
        let test_data = [
           (01, 10),
            //  (02, 100),
        //      (03, 200),
        //     (04, 500),
        //     (05, 1000),
        //    (06, 10_000),
        //     (07, 100_000),
        //      (08, 300_000),
        ];
        //do not change
        let array_a = vec![1., 1., 1.];
        let array_k = vec![0.1, 0.9, 0.3];

        for (step, freq) in test_data {
            let target = 1.;
            //log::debug!("amplitude: {}", target);
            let mut amplitude = Amplitude::new(freq); //(array_a/k.len());
            let mut angle = Angle::new(freq, 0.0);
            let mut angle_ = 0.;
            let mut result = 0.;

            loop{
                angle_ = angle.add();
                result = amplitude.calc(&array_a, &array_k, 72.).round();
                log::debug!("added angle and its amplitude: {}, {}", angle_* 180. / PI, result);
                if(angle_ >= Angle::PI2 - (Angle::PI2/freq as f64)* 1.5){
                    log::debug!("last added angle and its amplitude: {}, {}", angle_ * 180. / PI, result);
                    break;
                }
            }
            
            log::debug!("step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
            //assert!(result >= 0., "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
            //assert!(target == result, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        }
        test_duration.exit();
    }
}