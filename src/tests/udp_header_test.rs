#[cfg(test)]

mod tests {
    use std::{sync::Once, time::{Duration, Instant}};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};

    use crate::main_service::udp_header::UdpHeader;
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
        let test_data:[Vec<(u8, u8, u8, u8)>; 4] = [
            (vec![(0, 0, 16, 255)]),
            (vec![(1, 22, 32, 100)]),
            (vec![(0, 0, 16, 0)]),
            (vec![(1, 22, 64, 100)]),
        ];

        for (step, conf) in test_data.iter().enumerate(){
            let (syn, addr, r#type, count) = conf[0];
            let header = UdpHeader::new(syn, addr, r#type, count);
            println!("step: {}", step);
            println!("syn: {}, addr: {:?}, type: {:?}, count: {:?}\n", header.syn, header.addr, header.r#type, header.count);
            let result = header.to_bytes();
            let mut target = Vec::new();
            target.push(syn);
            target.push(addr);
            target.push(r#type);
            target.push(count);
            log::debug!("result: {:?}, target: {:?}", result, target);
            assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        }
        test_duration.exit();
    }
}
