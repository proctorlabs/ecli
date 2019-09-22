use super::*;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum Action {
    Nav(Nav), // These are navigation types without options, we box them here so they can be used without a hashmap
    Script {
        script: EcliTemplate,
        #[serde(default = "default_shell")]
        shell: EcliTemplate,
    },
    Command {
        command: EcliTemplate,
        #[serde(default)]
        args: Vec<EcliTemplate>,
    },
    Check {
        check: EcliExpression,
        pass: OneOrMany<String>,
        fail: OneOrMany<String>,
    },
    Prompt {
        prompt: EcliTemplate,
        val: String,
        #[serde(default)]
        password: bool,
    },
    Print {
        print: EcliTemplate,
    },
    Goto {
        goto: String,
    },
    Set {
        set: templar::Document,
    },
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Nav {
    Pop,
    Exit,
    Pause,
}

fn default_shell() -> EcliTemplate {
    "sh".into()
}
