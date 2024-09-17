#[cfg(test)]
//сделать класс и формулы для комплексного числа
mod amplitude {
    use std::{f64::consts::PI, sync::Once, time::Duration};
    use rand::Rng;
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};

    use crate::{amplitude::Amplitude, stuff::approx_eq::AproxEq};
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
    /// Calculating amplitude on static arrays
    fn formula(alpha: f64, array_a:&Vec<f64>, array_phi: &Vec<f64>) -> f64{
        log::debug!("alpha: {}", alpha);
        array_a[0]*(alpha + array_phi[0]).sin() + array_a[1]*(alpha + array_phi[1]).sin() + array_a[2]*(alpha + array_phi[2]).sin()
    }
    ///
    /// Testing amplitude on `exact` values
    #[test]
    fn exact_values() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let self_id = "test";
        log::debug!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();
        // Manually calculated values
        let test_data = [
            ( 90.*PI/180., 5.821146788337327), //(angle in radians, calculated amplitude)
            (234.*PI/180., -5.5227443516451995),
            (180.*PI/180., -1.3837326982209683),
            (270.*PI/180., -5.821146788337327),
            (360.*PI/180., 1.3837326982209674),
            (  0.*PI/180., 1.3837326982209692),
            ( 45.*PI/180., 5.094619142577222),
            ( 30.*PI/180., 4.1089210628752095),
        ];
        let conf: serde_yaml::Value = serde_yaml::from_str(r#"
            1.0: 0.1
            2.0: 0.2
            3.0: 0.3
        "#).unwrap();
        let amplitude = Amplitude::new(build_apmlitude_params(&conf));
        for (angle, target) in test_data {
            let result = amplitude.calc(angle);
            log::debug!("angle {:>7.3},  result: {:.5},  target: {:.5}", angle *180./PI, result, target);
            assert!(result.aprox_eq(target, 3), "angle {} \nresult: {:?}\ntarget: {:?}", angle *180./PI, result, target);
        }
        test_duration.exit();
    }
    ///
    /// 
    fn build_apmlitude_params(conf: &serde_yaml::Value) -> Vec<(f64, f64)> {
        match conf.as_mapping() {
            Some(conf) => {
                conf.into_iter().map(|(key, value)| {
                    let amp = key.as_f64().unwrap_or_else(|| panic!("build_apmlitude_params | Invalid f64 amplitude value '{:?}' in config {:#?}", key, conf));
                    let phi = value.as_f64().unwrap_or_else(|| panic!("build_apmlitude_params | Invalid f64 phi value '{:?}' in config {:#?}", value, conf));
                    (amp, phi)
                }).collect()
            }
            None => panic!("build_apmlitude_params | Invalid config {:#?}", conf),
        }
    }
    ///
    /// 
    fn cycle_formula(alpha: f64, params: &Vec<(f64, f64)>) -> f64{
        params.iter().fold(0.0, |value, (amp, phi)| {
            value + *amp * (alpha + *phi).sin()
        })
    }
    ///
    /// Testing amplitude on `random` values
    #[test]
    fn random_values(){
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let self_id = "test";
        log::debug!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();
        ////////////////////////////////////////////////// 
        // ============== Configurations ============== //
        // Sampling freq - defines steps of calculations: Δα = 2π / freq
        let sampling_freq = 100;
        let params_count = 3;
        // ============== Configurations ============== //
        ////////////////////////////////////////////////// 
        let mut rng = rand::thread_rng();
        let params = (0..params_count).fold(vec![], |mut params, i| {
            let amp = rng.gen_range(0.01..10.0);
            let phi = rng.gen_range(0.00..2.0 * PI);
            params.push((amp, phi));
            params
        });
        let conf = params.iter().fold(String::new(), |mut conf, (amp, phi)| {
            conf.push_str(&format!(r#"{}: {}
"#, amp, phi));
            conf
        });
        log::debug!("conf: {:#?}", conf);
        let conf: serde_yaml::Value = serde_yaml::from_str(&conf).unwrap();
        log::debug!("conf: {:#?}", conf);
        let test_data = (0..sampling_freq * 3).map(|step| {
            let alpha = (step as f64) * 2.0 * PI / (sampling_freq as f64);
            let value = cycle_formula(alpha, &params);
            (alpha, value)
        });
        let amplitude = Amplitude::new(build_apmlitude_params(&conf));
        for (angle, target) in test_data {
            log::debug!("amplitude: {}", target);
            let result = amplitude.calc(angle);
            log::debug!("angle {} \nresult: {:?}\ntarget: {:?}", angle *180./PI, result, target);
            assert!(result.aprox_eq(target, 3));
        }
        test_duration.exit();
    }
}