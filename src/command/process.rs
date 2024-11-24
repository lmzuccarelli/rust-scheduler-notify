use crate::api::schema::Service;
use crate::error::scheduler::SchedulerError;
use custom_logger::*;
use notify_rust::{Notification, Timeout};
use std::process::Command;

pub async fn execute_service(log: &Logging, service: Service) -> Result<(), SchedulerError> {
    let binary = format!("{}", service.binary);
    let mut output = Command::new(binary);
    if service.args.is_some() {
        let s = service.clone();
        for arg in s.args.unwrap().iter() {
            if arg.name.len() > 0 {
                output.arg(arg.name.clone());
            }
            if arg.value.len() > 0 {
                output.arg(arg.value.clone());
            }
        }
    }
    let res = output.output();
    if res.is_err() {
        return Err(SchedulerError::new(&format!(
            "{}",
            String::from_utf8_lossy(&res.unwrap().stderr)
        )));
    }
    if res.is_ok() {
        let response = format!("{}", String::from_utf8_lossy(&res.as_ref().unwrap().stdout));
        let err_response = format!("{}", String::from_utf8_lossy(&res.as_ref().unwrap().stderr));
        if err_response.contains("ERROR") {
            return Err(SchedulerError::new(&format!(
                "{}",
                String::from_utf8_lossy(&res.unwrap().stderr)
            )));
        }
        log.info(&response);
        log.warn(&err_response);
    }
    Ok(())
}

pub fn notification(title: String, body: String, icon: String) -> Result<(), SchedulerError> {
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
