use std::collections::HashMap;
use std::io::Read;
use tera::Value;

pub fn file_contents(
    val: Value,
    _: HashMap<String, Value>,
) -> std::result::Result<Value, tera::Error> {
    let val = match val {
        Value::String(s) => s,
        _ => val.to_string(),
    };
    let mut f = std::fs::File::open(val.to_string()).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    Ok(Value::String(s))
}
