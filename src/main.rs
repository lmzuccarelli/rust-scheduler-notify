use crate::api::schema::*;
use crate::command::process::{execute_service, notification};
use crate::config::load_and_parse::{load_config, parse_yaml_config};
use crate::error::scheduler::SchedulerError;
use chrono::{DateTime, Duration, Local};
use clap::Parser;
use cron::Schedule;
use custom_logger::{Level, Logging};
use std::process;
use std::str::FromStr;
use std::sync::mpsc;
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

    // add mpsc channel threading
    loop {
        let (tx, rx) = mpsc::channel();
        for service in sc.spec.services.iter() {
            let tx = tx.clone();
            let service = service.clone();
            thread::spawn(move || {
                let crn = Schedule::from_str(&service.cron).unwrap();
                let t = crn.upcoming(offset).take(1).nth(0).unwrap();
                tx.send(t).unwrap();
            });
        }

        let dt = Local::now();
        let naive_utc = dt.naive_utc();
        let dt_new = DateTime::<Local>::from_naive_utc_and_offset(naive_utc, offset);
        let dt_formated = dt_new.format("%Y-%m-%d %H:%M:%S");

        for service in sc.spec.services.iter() {
            let t = rx.recv().unwrap();
            let indx = t - Duration::seconds(1);
            let indx_fmt = indx.format("%Y-%m-%d %H:%M:%S");
            log.debug(&format!(
                "{:<20} => ttf {} : {}",
                service.name,
                indx_fmt.to_string(),
                dt_formated.to_string()
            ));
            if indx_fmt.to_string() == dt_formated.to_string() {
                let res = execute_service(log, service.clone()).await;
                if res.is_err() {
                    log.error(&format!(
                        "{}",
                        res.err().as_ref().unwrap().to_string().to_lowercase()
                    ));
                    process::exit(1);
                }
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
                    }
                }
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
