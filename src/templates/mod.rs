use crate::*;
use std::collections::HashMap;
use std::io::Read;
use std::sync::Mutex;
use tera::{Context, Tera, Value};

lazy_static! {
    pub static ref TERA: Mutex<Tera> = {
        let mut t = Tera::default();
        t.register_filter("contents", file_contents);
        t.autoescape_on(vec![]);
        Mutex::new(t)
    };
    pub static ref CONTEXT: Mutex<Context> = {
        let mut c = Context::default();
        let mut map: HashMap<String, String> = HashMap::default();
        map.insert("version".into(), crate_version!().into());
        c.insert("ecli", &map);
        Mutex::new(c)
    };
}

fn file_contents(val: Value, _: HashMap<String, Value>) -> std::result::Result<Value, tera::Error> {
    let val = match val {
        Value::String(s) => s,
        _ => val.to_string(),
    };
    let mut f = std::fs::File::open(val.to_string()).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    Ok(Value::String(s))
}

pub fn render(template: &str) -> Result<String> {
    let mut tera = TERA.lock().unwrap();
    let context = CONTEXT.lock().unwrap();
    tera.add_raw_template("current", template).unwrap();
    let res = tera.render("current", &*context).unwrap();
    Ok(res)
}

pub fn context_set(key: &str, val: &str) -> Result<()> {
    let mut context = CONTEXT.lock().unwrap();
    context.insert(key, val);
    Ok(())
}
