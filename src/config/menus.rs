use super::*;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum Menu {
    Choice(ChoiceMenu),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct ChoiceMenu {
    pub title: EcliTemplate,
    pub entries: Vec<Entry>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Entry {
    pub text: EcliTemplate,
    pub actions: OneOrMany<String>,
}
