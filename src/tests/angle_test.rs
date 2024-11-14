#[cfg(test)]

mod angle {
    use std::{f64::consts::PI, sync::Once, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{angle::Angle, stuff::approx_eq::AproxEq};
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
    /// Testing that `count` of steps is equal to `freq`
    #[test]
    fn check_step_count() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let self_id = "test";
        log::debug!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();
        let test_data = [
            (01, 10),
            (02, 100),
            (03, 200),
            (04, 500),
            (05, 1000),
            (06, 10_000),
            (07, 100_000),
            (08, 300_000),
        ];
        for (step, freq) in test_data {
            let target = freq;
            let mut count = 0;
            log::debug!("frequency: {}", target);
            let mut angle = Angle::new(freq, 0.0);
            let mut angle_ = 0.;
            count += 1; // adding the initial angle
            loop{
                angle_ = angle.add();
                count += 1;
                if(angle_ >= Angle::PI2 - (Angle::PI2/freq as f64)* 1.5){
                    log::debug!("last added angle: {}", angle_);
                    break;
                }
            }
            log::debug!("step {} \nresult: {:?}\ntarget: {:?}", step, count, target);
            assert!(target == count, "step {} \nresult: {:?}\ntarget: {:?}", step, count, target);
        }
        test_duration.exit();
    }    
    ///
    /// Testing that last `angle` is equal to `delta`
    #[test]
    fn check_last_angle() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let self_id = "test";
        log::debug!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();
        let test_data = [
            (01, 10),
            (02, 100),
            (03, 200),
            (04, 500),
            (05, 1000),
            (06, 10_000),
            (07, 100_000),
            (08, 300_000),
        ];
        for (step, freq) in test_data {
            let target = Angle::PI2 / (freq as f64);
            let target_grad = 180.0 * target / PI;
            let count = freq;
            let mut angle = Angle::new(freq, 0.0);
            log::debug!("delta: {} ({})", target, target_grad);
            let mut flag = false;
            for index in 0..count {
                let angle_ = angle.add();

                log::trace!("{} | angle: {} ({})", index, angle_, 180.0 * angle_ / PI);
                if angle_ >= (Angle::PI2 - target * 1.5) {
                    let result = Angle::PI2 - angle_;
                    let result_grad = 180.0 * result / PI;
                    log::debug!("angle_:{}", angle_);
                    log::debug!("step {} \nresult: {:?} ({})\ntarget: {:?} ({})", step, result, result_grad, target, target_grad);
                    assert!(result.aprox_eq(target, 3), "step {} \nresult: {:?} ({})\ntarget: {:?} ({})", step, result, result_grad, target, target_grad);
                    flag = true;
                }
            }
            assert!(flag == true, "step {} \nresult: {:?} \ntarget: {:?} ", step, flag, true);
        }
        test_duration.exit();
    }
}