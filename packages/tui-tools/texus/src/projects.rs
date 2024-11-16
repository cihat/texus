use std::fs;

use serde::Serialize;
use strum::Display;

#[derive(Default, Serialize, PartialEq, Eq, Clone, Display)]
pub enum ProjectStatus {
  #[default]
  Running,
  Stopped,
  Building,
}

#[derive(Default, Serialize, PartialEq, Eq, Clone)]
pub struct Project {
  pub name: String,
  pub status: ProjectStatus,
  pub description: String,
  pub dependencies: Vec<String>,
  pub commands: Vec<String>,
}

impl Project {
  pub fn new(
    name: String,
    description: String,
    dependencies: Vec<String>,
    commands: Vec<String>,
  ) -> Self {
    Self {
      name,
      description,
      dependencies,
      commands,
      ..Default::default()
    }
  }
  fn start(&mut self) {
    self.status = ProjectStatus::Running;
  }
}

fn get_package_json(path: &String) -> String {
  let package_json_path = format!("{}/package.json", path);
  let package_json = fs::read_to_string(package_json_path);

  if let Ok(json) = package_json {
    return json;
  } else {
    return "".to_string();
  }
}

fn get_folder_info(path: &String) -> Vec<Project> {
  fs::read_dir(path)
    .into_iter()
    .flat_map(|entries| entries.filter_map(Result::ok))
    .filter_map(|entry| {
      let path = entry.path();
      if path.is_dir() {
        let package_json = get_package_json(&path.to_string_lossy().to_string());
        let package_json: serde_json::Value = match serde_json::from_str(&package_json) {
          Ok(json) => json,
          Err(_) => return None,
        };

        let dependencies = package_json["dependencies"]
          .as_object()
          .map(|dependencies| dependencies.keys().map(|k| k.to_string()).collect())
          .unwrap_or_default();

        let commands = package_json["scripts"]
          .as_object()
          .map(|scripts| scripts.keys().map(|k| k.to_string()).collect())
          .unwrap_or_default();

        let project = Project::new(
          path.file_name()?.to_str().map(String::from)?,
          "Description of project 1".to_string(),
          dependencies,
          commands,
        );
        Some(project)
      } else {
        None
      }
    })
    .collect()
}

pub fn get_projects() -> Vec<Project> {
  let frontend_apps_path = "/Users/cihatsalik/www/frontend/packages/apps".to_string();

  get_folder_info(&frontend_apps_path)
}
