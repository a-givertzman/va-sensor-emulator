#[cfg(test)]

mod main_service_config {
    use std::{sync::Once, time::{Duration, Instant}};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};

    use crate::main_service::main_service_config::MainServiceConf;
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
    /// Testing MainServiceConf read from the yaml
    #[test]
    fn new() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        log::debug!("");
        let dbg_id = "test";
        log::debug!("\n{}", dbg_id);
        let test_duration = TestDuration::new(dbg_id, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (
                serde_yaml::from_str(r#"service MainService MainService-1:
                    address: 127.0.0.1:15181
                    sampl-freq: 100 ms
                    buf-size: 512
                    signal:
                        # freq: amplitude, [phase]
                        #  Hz     R.U.      rad
                        100:     100.11
                        220:     220.22     3.14
                "#).unwrap(),
                MainServiceConf {
                    addr: "address: 127.0.0.1:15181".to_owned(),
                }
            ),
        ];
        for (conf, target) in test_data {
            let result = MainServiceConf::from_yaml(dbg_id, &conf);
            assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        }
        test_duration.exit();
    }
}
