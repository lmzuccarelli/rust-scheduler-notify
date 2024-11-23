use clap::Parser;
use serde_derive::{Deserialize, Serialize};

/// rust-microservice-package-manager cli struct
#[derive(Parser)]
#[command(name = "rust-scheduler-notify")]
#[command(author = "Luigi Mario Zuccarelli <luzuccar@redhat.com>")]
#[command(version = "0.1.0")]
#[command(about = "A simple command line scheduler, reads a config and schedules the tasks with notify", long_about = None)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// set the loglevel
    #[arg(
        value_enum,
        short,
        long,
        value_name = "loglevel",
        default_value = "info",
        help = "Set the log level [possible values: info, debug, trace]"
    )]
    pub loglevel: Option<String>,

    #[arg(
        short,
        long,
        value_name = "config-file",
        help = "The config file used to schedule a task (required)"
    )]
    pub config_file: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SchedulerConfig {
    #[serde(rename = "apiVersion")]
    api_version: String,

    #[serde(rename = "kind")]
    kind: String,

    #[serde(rename = "spec")]
    pub spec: Spec,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Spec {
    #[serde(rename = "services")]
    pub services: Vec<Service>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Service {
    /// name must correspond to the actual binary that was created
    #[serde(rename = "name")]
    pub name: String,
    /// binary_path is the path to the actual microservice project on disk
    /// and the link to the binary
    #[serde(rename = "binary")]
    pub binary: String,

    /// cron - convert to scheduler format
    #[serde(rename = "cron")]
    pub cron: String,

    #[serde(rename = "summary")]
    pub summary: String,

    #[serde(rename = "body")]
    pub body: String,

    #[serde(rename = "icon")]
    pub icon: String,

    #[serde(rename = "env")]
    pub env: Option<Vec<KeyValue>>,

    #[serde(rename = "args")]
    pub args: Option<Vec<KeyValue>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KeyValue {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "value")]
    pub value: String,
}
