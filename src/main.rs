use crate::api::schema::*;
use crate::command::process::execute_service;
use crate::config::load_and_parse::{load_config, parse_yaml_config};
use crate::error::scheduler::SchedulerError;
use chrono::{DateTime, Duration, Local};
use clap::Parser;
use cron::Schedule;
use custom_logger::{Level, Logging};
use notify_rust::{Notification, Timeout};
use std::process;
use std::str::FromStr;
use std::time::SystemTime;

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

    // Start the scheduler
    loop {
        let dt = Local::now();
        let naive_utc = dt.naive_utc();
        let offset = dt.offset().clone();
        let dt_new = DateTime::<Local>::from_naive_utc_and_offset(naive_utc, offset);
        let dt_formated = dt_new.format("%Y-%m-%d %H:%M:%S");
        for service in sc.spec.services.iter() {
            let crn_res = Schedule::from_str(&service.cron);
            if crn_res.is_err() {
                log.error(&format!(
                    "{}",
                    crn_res.err().as_ref().unwrap().to_string().to_lowercase()
                ));
                process::exit(1);
            }
            let crn = crn_res.as_ref().unwrap();
            let start = SystemTime::now();
            // only interested in one future event
            for index in crn.upcoming(offset).take(1) {
                let indx = index - Duration::seconds(1);
                let indx_fmt = indx.format("%Y-%m-%d %H:%M:%S");
                if indx_fmt.to_string() == dt_formated.to_string() {
                    let _res = execute_service(log, service.clone()).await;
                    if service.notify {
                        let n_res = notification(
                            service.name.clone(),
                            service.body.clone(),
                            service.icon.clone(),
                        );
                        if n_res.is_err() {
                            log.error(&format!(
                                "{}",
                                n_res.err().as_ref().unwrap().to_string().to_lowercase()
                            ));
                            process::exit(1);
                        }
                    }
                }
            }
            let end = SystemTime::now();
            let duration = end.duration_since(start).unwrap();
            log.debug(&format!("service execution time {:?}", duration));
        }
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}

fn notification(title: String, body: String, icon: String) -> Result<(), SchedulerError> {
    let notify_res = Notification::new()
        .summary(title.as_ref())
        .body(&body.clone())
        .icon(&icon)
        .timeout(Timeout::Milliseconds(1000))
        .show();

    if notify_res.is_err() {
        return Err(SchedulerError::new(&format!(
            "[notification] {}",
            notify_res.err().unwrap().to_string().to_lowercase()
        )));
    }
    Ok(())
}
