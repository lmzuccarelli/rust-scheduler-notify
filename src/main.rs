use crate::api::schema::*;
use crate::command::process::{execute_service, notification};
use crate::config::load_and_parse::{load_config, parse_yaml_config};
use crate::error::scheduler::SchedulerError;
use chrono::{DateTime, Duration, Local};
use clap::Parser;
use config::load_and_parse::get_service;
use cron::Schedule;
use custom_logger::{Level, Logging};
use std::collections::HashMap;
use std::process;
use std::str::FromStr;
//use std::sync::mpsc;
use std::thread;

//use std::time::SystemTime;

mod api;
mod command;
mod config;
mod error;

#[tokio::main]
async fn main() -> Result<(), SchedulerError> {
    let args = Cli::parse();

    let lvl = args.loglevel.as_ref().unwrap();

    let l = match lvl.as_str() {
        "info" => Level::INFO,
        "debug" => Level::DEBUG,
        "trace" => Level::TRACE,
        _ => Level::INFO,
    };

    let log = &Logging { log_level: l };

    let config_res = load_config(args.config_file.to_string()).await;
    if config_res.is_err() {
        log.error(&format!(
            "{}",
            config_res
                .err()
                .as_ref()
                .unwrap()
                .to_string()
                .to_lowercase()
        ));
        process::exit(1);
    }

    let sc_res = parse_yaml_config(config_res.unwrap());
    if sc_res.is_err() {
        log.error(&format!(
            "{}",
            sc_res.err().as_ref().unwrap().to_string().to_lowercase()
        ));
        process::exit(1);
    }
    let sc = sc_res.unwrap();
    log.info("starting scheduler ...");

    let dt = Local::now();
    let offset = dt.offset().clone();
    let mut ttf_map: HashMap<String, String> = HashMap::new();

    // simplify cron threading
    loop {
        for service in sc.spec.services.iter() {
            if !service.skip {
                let crn = Schedule::from_str(&service.cron).unwrap();
                let ttf = crn.upcoming(offset).take(1).nth(0).unwrap();
                let ttf = ttf - Duration::seconds(1);
                let ttf_formated = ttf.format("%Y-%m-%d %H:%M:%S").to_string();
                ttf_map.insert(service.name.clone(), ttf_formated);
            }
        }

        let dt = Local::now();
        let naive_utc = dt.naive_utc();
        let dt_new = DateTime::<Local>::from_naive_utc_and_offset(naive_utc, offset);
        let dt_formated = dt_new.format("%Y-%m-%d %H:%M:%S");

        for (k, v) in ttf_map.clone().iter() {
            log.debug(&format!(
                "{:<20} => ttf {} : {}",
                k,
                v,
                dt_formated.to_string()
            ));
            if v.clone() == dt_formated.to_string() {
                let name = k.clone();
                let config = sc.clone();
                thread::spawn(move || {
                    let thread_log = &Logging {
                        log_level: Level::INFO,
                    };
                    let service = get_service(name, config);
                    // no error checking
                    if service.notify {
                        let _ = notification(
                            service.name.clone(),
                            service.body.clone(),
                            service.icon.clone(),
                        );
                    }
                    // fire and forget
                    // no error checking
                    let _ = execute_service(thread_log, service.clone());
                });
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
