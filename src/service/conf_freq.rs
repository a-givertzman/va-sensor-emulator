use std::{str::FromStr, time::Duration};
use log::trace;
use regex::RegexBuilder;
use serde::Deserialize;
///
/// Unit of Duration
/// -  Hz - Hertz, 
/// - kHz - Kilo Hertz, 
/// - MHz - Mega Hertz, 
/// - GHz - Mega Hertz, 
/// - THz - Mega Hertz, 
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum ConfFreqUnit {
    Nanos,
    Micros,
    Millis,
    Secs,
    Mins,
    Hours,
}
//
// 
impl FromStr for ConfFreqUnit {
    type Err = String;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "ns" => Ok(Self::Nanos),
            "us" => Ok(Self::Micros),
            "ms" => Ok(Self::Millis),
            "s" => Ok(Self::Secs),
            "m" => Ok(Self::Mins),
            "h" => Ok(Self::Hours),
            _ => Err(format!("ConfFreqUnit.from_str | Unknown duration unit: '{}'", input))
        }
    }
}


///
/// keyword konsists of 2 fields:
/// ```
/// | value  |  unit  |
/// |--------|--------|
/// | requir | opt    |
/// |--------|--------|
/// | 111    |  ns    | - 111 nanoseconds
/// | 12     |  us    | - 12 microseconds
/// | 11     |  ms    | - 11 milliseconds
/// | 5      |  s     | - 5 sec
/// | 5      |        | - 5 sec
/// | 3      |  m     | - 3 minutes
/// | 1      |  h     | - 1 hour
/// ````
#[derive(Debug, Deserialize, PartialEq)]
pub struct ConfDuration {
    value: u64,
    unit: ConfFreqUnit,
}
//
// 
impl ConfDuration {
    ///
    /// New instance if ConfDuration
    pub fn new(value: u64, unit: ConfFreqUnit) -> Self {
        Self {
            value,
            unit,
        }
    }
    ///
    /// 
    pub fn toDuration(&self) -> Duration {
        match self.unit {
            ConfFreqUnit::Nanos => Duration::from_nanos(self.value),
            ConfFreqUnit::Micros => Duration::from_micros(self.value),
            ConfFreqUnit::Millis => Duration::from_millis(self.value),
            ConfFreqUnit::Secs => Duration::from_secs(self.value),
            ConfFreqUnit::Mins => Duration::from_secs(self.value),
            ConfFreqUnit::Hours => Duration::from_secs(self.value),
        }
    }
}
//
// 
impl FromStr for ConfDuration {
    type Err = String;
    fn from_str(input: &str) -> Result<ConfDuration, String> {
        trace!("ConfDuration.from_str | input: {}", input);
        let re = r#"^[ \t]*(\d+)[ \t]*(ns|us|ms|s|m|h){0,1}[ \t]*$"#;
        let re = RegexBuilder::new(re).multi_line(true).build().unwrap();
        let groupValue = 1;
        let groupUnit = 2;
        match re.captures(input) {
            Some(caps) => {
                match &caps.get(groupValue) {
                    Some(first) => {
                        match first.as_str().parse() {
                            Ok(value) => {
                                let unit = match &caps.get(groupUnit) {
                                    Some(u) => match ConfFreqUnit::from_str(u.as_str()) {
                                        Ok(unit) => Ok(unit),
                                        Err(err) => Err(err),
                                    }
                                    None => Ok(ConfFreqUnit::Secs),
                                };
                                match unit {
                                    Ok(unit) => Ok(ConfDuration::new(value, unit)),
                                    Err(err) => Err(err),
                                }
                            }
                            Err(err) => Err(format!("ConfDuration.from_str | Error parsing duration value: '{}'\n\terror: {:?}", &input, err)),
                        }
                    }
                    None => Err(format!("ConfDuration.from_str | Error parsing duration value: '{}'", &input)),
                }
            }
            None => {
                Err(format!("ConfDuration.from_str | Error parsing duration value: '{}'", &input))
            }
        }
    }
}