#[cfg(test)]

mod udp_header {
    use std::{sync::Once, time::{Duration, Instant}};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::main_service::upd_header::UdpHeader;
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
    fn test_to_bytes() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        log::debug!("");
        let self_id = "test";
        log::debug!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data: [((u8, u8, u8, u8), Vec<u8>); 2] = [
            ((0, 0, 16, 255), vec![0, 0, 16, 255]),
            ((0, 0, 0, 0), vec![0, 0, 0, 0]),
        ];

        for (step, (data, target)) in test_data.iter().enumerate(){
            let header = UdpHeader::new(data.0, data.1, data.2, data.3);
            log::debug!("syn: {}, addr: {:?}, type: {:?}, count: {:?}\n", header.syn, header.addr, header.r#type, header.count);
            let result = header.to_bytes();
            log::debug!("result: {:?}, target: {:?}", result, target);
            assert!(result == *target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        }
        test_duration.exit();
    }
}