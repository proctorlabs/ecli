use super::*;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum Action {
    Nav(Nav), // These are navigation types without options, we box them here so they can be used without a hashmap
    Script {
        script: String,
        #[serde(default = "default_shell")]
        shell: String,
    },
    Command {
        command: String,
        #[serde(default)]
        args: Vec<String>,
    },
    Check {
        check: String,
        pass: OneOrMany<String>,
        fail: OneOrMany<String>,
    },
    Prompt {
        prompt: String,
        val: String,
        #[serde(default)]
        password: bool,
    },
    Print {
        print: String,
    },
    Goto {
        goto: String,
    },
    Set {
        set: Value,
    },
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Nav {
    Pop,
    Exit,
    Pause,
}

fn default_shell() -> String {
    "sh".into()
}
