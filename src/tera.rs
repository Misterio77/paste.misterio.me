use rocket_dyn_templates::{tera, Engines};
use std::collections::HashMap;
use std::env;

fn exec_path(_: &HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
    env::current_exe()
        .map(|p| tera::Value::String(p.to_string_lossy().into_owned()))
        .map_err(tera::Error::msg)
}

pub fn customize(engines: &mut Engines) {
    engines.tera.register_function("exec_path", exec_path);
}
