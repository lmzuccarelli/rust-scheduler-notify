use crate::api::schema::*;
use crate::error::scheduler::SchedulerError;
use std::fs::read_to_string;

// read the 'image set config' file
pub async fn load_config(config_file: String) -> Result<String, SchedulerError> {
    // Create a path to the desired file
    let data = read_to_string(config_file.clone());
    if data.is_err() {
        return Err(SchedulerError::new(&format!(
            "[load_config] {}",
            data.err().unwrap().to_string().to_lowercase()
        )));
    }
    Ok(data.unwrap())
}

// parse the 'image set config' file
pub fn parse_yaml_config(data: String) -> Result<SchedulerConfig, SchedulerError> {
    // Parse the string of data into serde_json::SchedulerConfig.
    let res = serde_yaml::from_str(&data);
    if res.is_err() {
        return Err(SchedulerError::new(&format!(
            "[parse_yaml_config] {}",
            res.err().unwrap().to_string().to_lowercase()
        )));
    }
    let root: SchedulerConfig = res.unwrap();
    Ok(root)
}

// get a specific service
#[allow(unused)]
pub fn get_service(service: String, config: SchedulerConfig) -> Service {
    let index = config
        .spec
        .services
        .iter()
        .position(|r| r.name == service)
        .unwrap();
    return config.spec.services[index].clone();
}
