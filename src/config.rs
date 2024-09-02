use serde::Deserialize;
use std::fs;
use std::path::Path;
use std::vec::Vec;
use strum_macros::EnumString;

// Example config:
//
//   check-on-startup = true
//
//   [defaults]
//   timeout = 5000
//   urgency = "normal"
//   notify-on = ["failed"]
//
//   # Match in order, first wins
//   [[services]]
//   name = "postfix"
//   notify-on = ["success", "failed"]
//   urgency = "urgent"
//
//   [[services]]
//   name = "foo*"
//   notify-on = []  # Ignore
//
//   [[services]]
//   name = "*"   # catch all
//   timeout = 5000
//   notify-on = ["failed"]

pub fn load(filename: &Path) -> eyre::Result<Config> {
    let contents = fs::read_to_string(filename)?;
    Ok(toml::from_str::<Config>(&contents)?)
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Config {
    #[serde(default = "default_true")]
    pub check_on_startup: bool,

    #[serde(default = "default_defaults")]
    pub defaults: Defaults,

    #[serde(default = "default_services")]
    pub services: Vec<Service>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Defaults {
    #[serde(default = "default_timeout")]
    pub timeout: u32, // ms

    #[serde(default = "default_urgency")]
    pub urgency: Urgency,

    #[serde(default = "default_notify_on")]
    pub notify_on: Vec<JobResult>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Service {
    pub name: String,

    #[serde(default = "default_timeout")]
    pub timeout: u32, // ms

    #[serde(default = "default_urgency")]
    pub urgency: Urgency,

    #[serde(default = "default_notify_on")]
    pub notify_on: Vec<JobResult>,
}

#[derive(Deserialize, Debug, PartialEq, EnumString)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Urgency {
    Low = 0,
    Normal = 1,
    Critical = 2,
}

#[derive(Deserialize, Debug, PartialEq, EnumString)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum JobResult {
    Done,
    Canceled,
    Timeout,
    Failed,
    Dependency,
    Skipped,
}

fn default_defaults() -> Defaults {
    Defaults {
        timeout: default_timeout(),
        urgency: default_urgency(),
        notify_on: default_notify_on(),
    }
}

fn default_timeout() -> u32 {
    5000
}

fn default_true() -> bool {
    true
}

fn default_urgency() -> Urgency {
    Urgency::Normal
}

fn default_notify_on() -> Vec<JobResult> {
    // XXX review this
    vec![JobResult::Failed]
}

fn default_services() -> Vec<Service> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn urgency_from_str() {
        assert_eq!(Urgency::from_str("low").unwrap(), Urgency::Low);
        assert_eq!(Urgency::from_str("critical").unwrap(), Urgency::Critical);
    }

    #[test]
    fn jobresult_from_str() {
        assert_eq!(JobResult::from_str("timeout").unwrap(), JobResult::Timeout);
        assert_eq!(JobResult::from_str("skipped").unwrap(), JobResult::Skipped);
    }

    #[test]
    fn empty() {
        let config: Config = toml::from_str("").unwrap();
        assert!(config.check_on_startup);
        assert_eq!(
            config.defaults,
            Defaults {
                timeout: 5000,
                urgency: Urgency::Normal,
                notify_on: vec![JobResult::Failed],
            }
        );
    }

    #[test]
    fn partial_defaults_section() {
        let config: Config = toml::from_str(
            r#"
            [defaults]
            timeout = 200
        "#,
        )
        .unwrap();
        assert_eq!(
            config.defaults,
            Defaults {
                timeout: 200,
                urgency: Urgency::Normal,
                notify_on: vec![JobResult::Failed],
            }
        );
    }

    #[test]
    fn defaults_urgency() {
        let config: Config = toml::from_str(
            r#"
            [defaults]
            urgency = "critical"
        "#,
        )
        .unwrap();
        assert_eq!(config.defaults.urgency, Urgency::Critical);
    }

    #[test]
    fn minimal_service() {
        let config: Config = toml::from_str(
            r#"
            [[services]]
            name = "foo"
        "#,
        )
        .unwrap();
        assert_eq!(config.services.len(), 1);
        let service = config.services.first().unwrap();
        assert_eq!(
            service,
            &Service {
                name: String::from("foo"),
                timeout: 5000,
                urgency: Urgency::Normal,
                notify_on: vec![JobResult::Failed],
            }
        )
    }

    #[test]
    fn maximal_service() {
        let config: Config = toml::from_str(
            r#"
            [[services]]
            name = "foo"
            timeout = 123
            urgency = "low"
            notify_on = ["done", "skipped"]
        "#,
        )
        .unwrap();
        assert_eq!(config.services.len(), 1);
        let service = config.services.first().unwrap();
        assert_eq!(
            service,
            &Service {
                name: String::from("foo"),
                timeout: 123,
                urgency: Urgency::Low,
                notify_on: vec![JobResult::Done, JobResult::Skipped],
            }
        )
    }

    #[test]
    fn three_services() {
        let config: Config = toml::from_str(
            r#"
            [[services]]
            name = "one"

            [[services]]
            name = "two"

            [[services]]
            name = "three"
        "#,
        )
        .unwrap();

        let names: Vec<String> = config.services.into_iter().map(|s| s.name).collect();
        assert_eq!(names, vec!["one", "two", "three"]);
    }

    #[test]
    fn service_without_name() {
        let err = toml::from_str::<Config>(
            r#"
            [[services]]
        "#,
        )
        .unwrap_err();
        assert_eq!(err.message(), "missing field `name`");
    }
}
