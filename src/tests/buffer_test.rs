#[cfg(test)]

mod buffer {
    use std::{sync::Once, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};

    use crate::buffer::Buffer;
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
    fn new() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let self_id = "buffer::new";
        log::debug!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();
        let test_data = [
            0.0,
            1.0,
            2.0,
            3.0,
            4.0,
            5.0,
            6.0,
            7.0,
            8.0,
            9.0,
        ];
        let len = test_data.len();
        let mut buffer = Buffer::new(len);
        for (step, angle) in test_data.iter().enumerate() {
            let result = buffer.add(*angle);
            if step < (len - 1) {
                // when step < 9
                let target = None;
                assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
            } 
            else {
                // when step == 9, we add last element and return full array
                let target = Some(test_data.iter().map(|value| *value as i16).collect());
                log::trace!("step: {}, angle: {}, result: {:?}", step, angle, result);
                log::trace!("target cur:{:?}", target);
                assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
            }
        }
        test_duration.exit();
    }
}