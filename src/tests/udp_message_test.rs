#[cfg(test)]

mod tests {
    use std::{sync::Once, time::{Duration, Instant}};
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
    fn test_udp_header() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        log::debug!("");
        let self_id = "test";
        log::debug!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (UdpHeader::new(0, 0, 64, 255), vec![]),
            (UdpHeader::new(22, 128, 16, 128), vec![89, 60, 0]),
            (UdpHeader::new(12, 67, 32, 78), vec![0, 9]),
            (UdpHeader::new(7, 9, 128, 0), vec![0, 0, 0]),
        ];

        for(step, (header, data)) in test_data.into_iter().enumerate(){
            let message = UpdMessage::new(UdpHeader::new(header.syn, header.addr, header.r#type, header.count), data.clone());
            let result = message.message();
            let mut target = header.to_bytes();
            target.extend_from_slice(&data);
            println!("step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
            log::debug!("step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
            assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        }
        test_duration.exit();
    }
}
