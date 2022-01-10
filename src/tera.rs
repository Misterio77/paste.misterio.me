use rocket_dyn_templates::{tera, Engines};
use std::collections::HashMap;
use std::env;
use chrono::{DateTime, Utc};

fn exec_path(_: &HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
    env::current_exe()
        .map(|p| tera::Value::String(p.to_string_lossy().into_owned()))
        .map_err(tera::Error::msg)
}

fn timestamp_sec(value: &tera::Value, _: &HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
    if let tera::Value::String(text) = value {
        let dt: DateTime<Utc> = text
            .parse()
            .map_err(|_| tera::Error::msg("Couldn't parse datetime string."))?;
        Ok(tera::Value::Number(tera::Number::from(dt.timestamp())))
    } else {
        Err(tera::Error::msg("Not a string."))
    }
}

fn timestamp_nano(value: &tera::Value, _: &HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
    if let tera::Value::String(text) = value {
        let dt: DateTime<Utc> = text
            .parse()
            .map_err(|_| tera::Error::msg("Couldn't parse datetime string."))?;
        Ok(tera::Value::Number(tera::Number::from(dt.timestamp_nanos())))
    } else {
        Err(tera::Error::msg("Not a string."))
    }
}

pub fn customize(engines: &mut Engines) {
    engines.tera.register_function("exec_path", exec_path);
    engines.tera.register_filter("timestamp_sec", timestamp_sec);
    engines.tera.register_filter("timestamp_nano", timestamp_nano);
}
