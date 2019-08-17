use std::process::Command;
use tera::{GlobalFn, Map, Result, Value};

pub fn make_exec() -> GlobalFn {
    Box::new(move |f| -> Result<Value> {
        let mut map = Map::new();
        let t = Value::Array(vec![]);
        let cmd = f.get("command").unwrap_or(&Value::Null).clone();
        let mut args = f.get("args").unwrap_or(&t).clone();
        if let Value::String(s) = args {
            args = Value::Array(vec![Value::String(s)]);
        };
        if let (Value::String(c), Value::Array(a)) = (cmd, args) {
            let args: Vec<String> = a
                .into_iter()
                .map(|aa| {
                    if let Value::String(s) = aa {
                        s.to_string()
                    } else {
                        aa.to_string()
                    }
                })
                .collect();
            match Command::new(c).args(args).output() {
                Ok(output) => {
                    map.insert("status".into(), output.status.to_string().into());
                    map.insert(
                        "stdout".into(),
                        String::from_utf8(output.stdout).unwrap_or_default().into(),
                    );
                    map.insert(
                        "stderr".into(),
                        String::from_utf8(output.stderr).unwrap_or_default().into(),
                    );
                }
                Err(e) => {
                    map.insert("status".into(), "-1".into());
                    map.insert("error".into(), e.to_string().into());
                }
            }
        } else {
            map.insert("status".into(), "-1".into());
        }
        Ok(Value::Object(map))
    })
}
