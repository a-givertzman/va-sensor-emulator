#[cfg(test)]

mod udp_message {
    use std::{sync::Once, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::main_service::udp_header::UdpHeader;
    use crate::main_service::udp_message::UpdMessage;
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
    fn build() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        log::debug!("");
        let self_id = "test";
        log::debug!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            ((0, 0, 64, 255), vec![], vec![0, 0, 64, 0, 0, 0, 255]),
            ((22, 127, 16, 128), vec![89, 60, 0], vec![22, 127, 16, 0, 0, 0, 128, 89, 60, 0]),
            ((12, 67, 32, 78), vec![0, 9], vec![12, 67, 32, 0, 0, 0, 78, 0, 9]),
            ((7, 9, 128, 0), vec![0, 0, 0], vec![7, 9, 128, 0, 0, 0, 0, 0, 0, 0]),
        ];
        for(step, (header, data, target)) in test_data.into_iter().enumerate(){
            let message = UpdMessage::new(UdpHeader::new(header.0, header.1, header.2, header.3), data.clone());
            let result = message.build();
            log::debug!("step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
            assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        }
        test_duration.exit();
    }
}
