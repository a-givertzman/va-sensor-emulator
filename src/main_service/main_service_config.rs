use log::info;
use regex::RegexBuilder;
use serde_yaml::to_string;
use std::{fs, time::Duration};
use sal_sync::services::{
    conf::conf_tree::ConfTree, entity::name::Name,
};
use crate::service::service_config::ServiceConfig;
///
/// creates config from serde_yaml::Value of following format:
/// ```yaml
/// service MainService MainService-1:
///     address: 127.0.0.1:15181
///     sampl-freq: 100 ms
///     buf-size: 512
///     signal:
///         # freq: amplitude, [phase]
///         #  Hz     R.U.      rad
///         100:     100.11
///         220:     220.22     3.14
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct MainServiceConf {
    pub name: Name,
    pub addr: String,
    pub sampl_freq: Option<Duration>,
    pub buf_size: u32,
    pub signal: Vec<(f64, f64, f64)>,
}
//
// 
impl MainServiceConf {
    ///
    /// - 
    pub fn new(parent: impl Into<String>, conf_tree: &mut ConfTree) -> MainServiceConf {
        log::trace!("TaskConfig.new | confTree: {:?}", conf_tree);
        let dbg_id = format!("MainServiceConf({})", conf_tree.key);
        let mut self_conf = ServiceConfig::new(&dbg_id, conf_tree.clone());
        log::trace!("{}.new | selfConf: {:?}", dbg_id, self_conf);
        let self_name = match self_conf.sufix().is_empty() {
            true => Name::new(parent, self_conf.name()),
            false => Name::new(parent, self_conf.sufix()),
        };
        log::debug!("{}.new | name: {:?}", dbg_id, self_name);
        let addr = self_conf.get_param_value("address").unwrap().as_str().unwrap().to_owned();
        log::debug!("{}.new | address: {:?}", dbg_id, addr);
        let sampl_freq = self_conf.get_duration("sampl-freq");
        log::debug!("{}.new | cycle: {:?}", dbg_id, sampl_freq);
        let buf_size = self_conf.get_param_value("buf-size").unwrap().as_u64().unwrap() as u32;
        log::debug!("{}.new | buf_size: {:?}", dbg_id, buf_size);
        let signal_conf = self_conf.get_param_value("signal").unwrap();
        let signal_conf = signal_conf.as_mapping().unwrap();
        log::debug!("{}.new | buf_size: {:?}", dbg_id, buf_size);
        let mut signal_index = 0;
        let mut signal = vec![];
        for (freq, amp_phi) in signal_conf.iter() {
            let freq = freq.as_f64().unwrap();
            let (amp, phi) = Self::amp_phi_from_str(&dbg_id, amp_phi);
            log::trace!("{}.new | signal[{}]: \tfreq: {:?}, \tamp:  {}, \tphase: {} rad", dbg_id, signal_index, freq, amp, phi);
            signal_index += 1;
            signal.push((
                freq,
                amp,
                phi
            ));
        }
        MainServiceConf {
            name: self_name,
            addr,
            sampl_freq,
            buf_size,
            signal,
        }
    }
    ///
    /// 
    fn amp_phi_from_str(dbg_id: &str, input: &serde_yaml::Value) -> (f64, f64) {
        let re = r#"^[ \t]*(\d+(?:\.\d+)*)(?:[ \t]+(\d+(?:\.\d+)*))*"#;
        let re = RegexBuilder::new(re).multi_line(false).build().unwrap();
        let group_amp = 1;
        let group_phi = 2;

        //знак переноса строки был из-за некоректного конвертирования value в string
        let input = match input {
            serde_yaml::Value::Number(val) => val.to_string(),
            serde_yaml::Value::String(val) => val.to_owned(),
            _ => panic!("{}.amp_phi_from_str | Amplitude parsing error in {:?}", dbg_id, input),
        };
        match re.captures(&input) {
            Some(caps) => {
                match &caps.get(group_amp) {
                    Some(first) => {
                        let amp: f64 = first.as_str().parse().unwrap();
                        match &caps.get(group_phi) {
                            Some(first) => {
                                let phi:f64 = first.as_str().parse().unwrap();
                                (amp, phi)
                            }
                            None => (amp, 0.0),
                        }
                    }
                    None => panic!("{}.amp_phi_from_str | Amplitude parsing error in {:?}", dbg_id, input),
                }
            }
            None => panic!("{}.amp_phi_from_str | Amplitude and Phase parsing error in {:?}", dbg_id, input),
        }
    }
    ///
    /// creates config from serde_yaml::Value of following format:
    pub(crate) fn from_yaml(parent: impl Into<String>, value: &serde_yaml::Value) -> MainServiceConf {
        match value.as_mapping().unwrap().into_iter().next() {
            Some((key, value)) => {
                Self::new(parent, &mut ConfTree::new(key.as_str().unwrap(), value.clone()))
            }
            None => {
                panic!("MainServiceConf.from_yaml | Format error or empty conf: {:#?}", value)
            }
        }        
    }
    ///
    /// reads config from path
    #[allow(dead_code)]
    pub fn read(parent: impl Into<String>, path: &str) -> MainServiceConf {
        match fs::read_to_string(path) {
            Ok(yaml_string) => {
                match serde_yaml::from_str(&yaml_string) {
                    Ok(config) => {
                        MainServiceConf::from_yaml(parent, &config)
                    }
                    Err(err) => {
                        panic!("MainServiceConf.read | Error in config: {:?}\n\terror: {:?}", yaml_string, err)
                    }
                }
            }
            Err(err) => {
                panic!("MainServiceConf.read | File {} reading error: {:?}", path, err)
            }
        }
    }
}