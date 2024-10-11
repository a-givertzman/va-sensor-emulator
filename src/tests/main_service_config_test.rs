#[cfg(test)]

mod main_service_config {
    use std::{sync::Once, time::{Duration, Instant}};
    use sal_sync::services::entity::name::Name;
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
                serde_yaml::from_str(r#"service MainService:
                    address: 127.0.0.1:15181
                    sampl-freq: 100 ms
                    buf-size: 512
                    signal:
                        # freq: amplitude, [phase]
                        #  Hz     R.U.      rad
                        100:     100.11     
                        220:     220.22     3.14
                        230:     230     3.14
                        231:     231     3
                        240:     240
                "#).unwrap(),
                MainServiceConf {
                    name: dbg_id.into(),
                    addr: "127.0.0.1:15181".to_owned(),
                    sampl_freq: Some(Duration::from_millis(100)),
                    buf_size: 512,
                    signal: vec![
                        (100., 100.11, 0.0),
                        (220., 220.22, 3.14),
                        (230., 230.00, 3.14),
                        (231., 231.00, 3.00),
                        (240., 240.00, 0.00),
                    ],
                }
            ),
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
                        230:     230     3.14
                        231:     231     3
                        240:     240
                "#).unwrap(),
                MainServiceConf {
                    // name: dbg_id.into(),
                    name: Name::new(dbg_id, "MainService-1"),
                    addr: "127.0.0.1:15181".to_owned(),
                    sampl_freq: Some(Duration::from_millis(100)),
                    buf_size: 512,
                    signal: vec![
                        (100., 100.11, 0.0),
                        (220., 220.22, 3.14),
                        (230., 230.00, 3.14),
                        (231., 231.00, 3.00),
                        (240., 240.00, 0.00),
                    ],
                }
            ),
        ];
        for (step, (conf, target)) in test_data.into_iter().enumerate() {
            let result = MainServiceConf::from_yaml(dbg_id, &conf);
            log::debug!("result: {:?}, target: {:?}", result, target);
            assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        }
        test_duration.exit();
    }
}
