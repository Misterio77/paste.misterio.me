use chrono::{DateTime, Utc};
use chrono_humanize::HumanTime;
use rocket_dyn_templates::{tera, Engines};

use std::collections::HashMap;
use std::env;

use tera::Value;

fn exec_path(_: &HashMap<String, Value>) -> tera::Result<Value> {
    env::current_exe()
        .map(|p| Value::String(p.to_string_lossy().into_owned()))
        .map_err(tera::Error::msg)
}

fn timestamp(value: &Value, _: &HashMap<String, Value>) -> tera::Result<Value> {
    if let Value::String(text) = value {
        let dt: DateTime<Utc> = text
            .parse()
            .map_err(|_| tera::Error::msg("Couldn't parse datetime string."))?;
        Ok(Value::Number(tera::Number::from(dt.timestamp())))
    } else {
        Err(tera::Error::msg("Not a string."))
    }
}

fn humanize(value: &Value, _: &HashMap<String, Value>) -> tera::Result<Value> {
    if let Value::String(text) = value {
        let dt: DateTime<Utc> = text
            .parse()
            .map_err(|_| tera::Error::msg("Couldn't parse datetime string."))?;
        Ok(Value::String(HumanTime::from(dt).to_string()))
    } else {
        Err(tera::Error::msg("Not a string."))
    }
}

fn version(_: &HashMap<String, Value>) -> tera::Result<Value> {
    Ok(Value::String(crate::VERSION.to_string()))
}

pub fn customize(engines: &mut Engines) {
    engines.tera.register_function("exec_path", exec_path);
    engines.tera.register_function("version", version);
    engines.tera.register_filter("timestamp", timestamp);
    engines.tera.register_filter("humanize", humanize);
}
