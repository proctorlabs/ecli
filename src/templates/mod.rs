use crate::*;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Mutex;
use tera::{Context, Tera};

mod filters;
mod functions;

lazy_static! {
    pub static ref ENV: HashMap<String, String> = {
        let mut res = HashMap::new();
        for (key, value) in std::env::vars() {
            res.insert(key, value);
        }
        res
    };
    pub static ref TERA: Mutex<Tera> = {
        let mut t = Tera::default();
        t.register_filter("contents", filters::file_contents);
        t.autoescape_on(vec![]);
        Mutex::new(t)
    };
    pub static ref CONTEXT: Mutex<Context> = {
        let mut c = Context::default();
        let mut map: HashMap<String, String> = HashMap::default();
        map.insert("version".into(), crate_version!().into());
        c.insert("ecli", &map);
        c.insert("env", &*ENV);
        Mutex::new(c)
    };
}

pub fn render(template: &str) -> Result<String> {
    let mut tera = TERA
        .lock()
        .map_err(|e| AppError::Fatal(format!("Tera lock poisoned. {}", e)))?;
    let context = CONTEXT
        .lock()
        .map_err(|e| AppError::Fatal(format!("Context lock poisoned. {}", e)))?;
    tera.add_raw_template("current", template)
        .map_err(|e| AppError::Info(format!("Failed to parse template! {}", e)))?;
    let res = tera
        .render("current", &*context)
        .map_err(|e| AppError::Info(format!("Failed to run template! {}", e)))?;
    Ok(res)
}

pub fn context_set<T: Serialize + ?Sized>(key: &str, val: &T) -> Result<()> {
    let mut context = CONTEXT.lock().unwrap();
    context.insert(key, val);
    Ok(())
}
