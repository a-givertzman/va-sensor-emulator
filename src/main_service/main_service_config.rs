use indexmap::IndexMap;
use log::{trace, debug};
use sal_sync::services::{conf::conf_tree::ConfTree, entity::{name::Name, point::point_config::PointConfig}, subscription::conf_subscribe::ConfSubscribe};
use std::{fs, time::Duration};
///
/// creates config from serde_yaml::Value of following format:
/// ```yaml
/// service MainService MainService-1:
///     cycle: 100 ms
///     buf-size: 512
///     signal:
///         # freq: amplitude, [phase]
///         #  Hz     R.U.      rad
///         - 100:  100.11
///         - 220:  220.22
/// 
#[derive(Debug, PartialEq, Clone)]
pub struct MainServiceConf {
    pub name: Name,
    pub cycle: Option<Duration>,
    pub buf_size: i64,
    pub signal: Vec<(f64, f64, f64)>,
    pub vars: Vec<String>,
}
//
// 
impl MainServiceConf {
    ///
    /// - 
    pub fn new(parent: impl Into<String>, conf: serde_yaml::Value) -> MainServiceConf {
        println!();
        trace!("MainServiceConf.new | conf: {:?}", conf);
        let mut vars = vec![];
        let self_id = format!("MainServiceConf({})", conf_tree.key);
        let mut self_conf = ServiceConfig::new(&self_id, conf_tree.clone());
        trace!("{}.new | selfConf: {:?}", self_id, self_conf);
        let self_name = Name::new(parent, self_conf.sufix());
        debug!("{}.new | name: {:?}", self_id, self_name);
        let cycle = self_conf.get_duration("cycle");
        debug!("{}.new | cycle: {:?}", self_id, cycle);
        let (rx, rx_max_length) = self_conf.get_in_queue().unwrap();
        debug!("{}.new | RX: {},\tmax-length: {:?}", self_id, rx, rx_max_length);
        let subscribe = ConfSubscribe::new(self_conf.get_param_value("subscribe").unwrap_or(serde_yaml::Value::Null));
        debug!("{}.new | sudscribe: {:#?}", self_id, subscribe);
        let mut node_index = 0;
        let mut nodes = IndexMap::new();
        for key in &self_conf.keys {
            let node_conf = self_conf.get(key).unwrap();
            trace!("{}.new | nodeConf: {:?}", self_id, node_conf);
            node_index += 1;
            let node_conf = FnConfig::new(&self_name.join(), &self_name, &node_conf, &mut vars);
            nodes.insert(
                format!("{}-{}", node_conf.name(), node_index),
                node_conf,
            );
        }
        MainServiceConf {
            name: self_name,
            cycle,
            rx,
            rx_max_length,
            subscribe,
            nodes,
            vars,
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
    ///
    /// Returns list of configurations of the defined points
    pub fn points(&self) -> Vec<PointConfig> {
        self.nodes.iter().fold(vec![], |mut points, (_node_name,node_conf)| {
            points.extend(node_conf.points());
            points
        })
    }
}