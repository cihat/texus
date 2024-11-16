use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize)]
pub enum ProjectScript {
  Start,
  Build,
}

#[derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize)]
//TODO: Implement this exec commands
pub enum ProjectCommand {
  Stop,
  StopAll,
}

impl ProjectScript {
  pub fn to_string(&self) -> String {
    match self {
      ProjectScript::Start => "start".to_string(),
      ProjectScript::Build => "build".to_string(),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize)]
pub enum Action {
  Tick,
  Render,
  Resize(u16, u16),
  Suspend,
  Resume,
  Quit,
  ClearScreen,
  Error(String),
  Help,
  ProjectScript(ProjectScript),
  ProjectCommand(ProjectCommand),
}
