use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Scene {
    pub id: String,
    pub text: String,
    pub choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub text: String,
    pub action: Option<Action>,
    pub target: String,
}

#[derive(Debug, Deserialize)]
pub struct Story {
    pub scene: Vec<Scene>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Action {
    pub variable: String,
    pub action_type: ActionType,
    pub value: usize,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ActionType {
    Add,
    Subtract,
    Set,
}
