#[cfg(test)]

mod main_service_config {
    use std::{sync::Once, time::{Duration, Instant}};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};

    use crate::main_service::main_service_config::MainServiceConf;
    // Определяем ComparisonConf
     #[derive(Debug, PartialEq)]
     struct ComparisonConf {
        pub addr: String,
        pub sampl_freq: Option<Duration>,
        pub buf_size: u64,
        pub signal: Vec<(f64, f64, f64)>,
     }
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
                    name: dbg_id.into(),
                    addr: "127.0.0.1:15181".to_owned(),
                    sampl_freq: Some(Duration::from_millis(100)),
                    buf_size: 512,
                    signal: vec![
                        (100., 100.11, 0.0),
                        (220., 220.22, 3.14)
                    ],
                }
            ),
        ];
        let mut step = 0;
        for (step, (conf, target)) in test_data.into_iter().enumerate() {
            let result = MainServiceConf::from_yaml(dbg_id, &conf);
            log::debug!("result: {:?}, target: {:?}", result, target);
            //временная структура для сравнения
            // let result_comp = ComparisonConf {
            //     addr: result.addr.trim_end().to_string(),
            //     sampl_freq: result.sampl_freq,
            //     buf_size: result.buf_size,
            //     signal: result.signal,
            // };
            
            // let target_comp = ComparisonConf {
            //     addr: target.addr,
            //     sampl_freq: target.sampl_freq,
            //     buf_size: target.buf_size,
            //     signal: target.signal,
            // };
            assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        }
        test_duration.exit();
    }
}
