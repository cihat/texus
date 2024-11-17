use serde::Serialize;
use std::{fs, os::unix::net::UnixStream, path::PathBuf};
use strum::Display;

#[derive(Default, Serialize, PartialEq, Eq, Clone, Display)]
pub enum ProjectStatus {
  #[default]
  Idle,
  Running,
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
    status: ProjectStatus,
  ) -> Self {
    Self {
      name,
      description,
      dependencies,
      commands,
      status,
    }
  }
}

pub struct ProjectManager {
  base_path: PathBuf,
}

impl ProjectManager {
  pub fn new(base_path: PathBuf) -> Self {
    Self { base_path }
  }

  fn read_file(path: &PathBuf) -> Option<String> {
    fs::read_to_string(path).ok()
  }

  fn check_running(webpack_sock_file_path: &PathBuf) -> ProjectStatus {
    if fs::metadata(webpack_sock_file_path).is_ok() {
      if let Ok(stream) = UnixStream::connect(webpack_sock_file_path) {
        drop(stream);
        return ProjectStatus::Running;
      }
    }
    ProjectStatus::Idle
  }

  fn parse_package_json(package_json: &str) -> (Vec<String>, Vec<String>) {
    let package_json: serde_json::Value = serde_json::from_str(package_json).unwrap_or_default();

    let dependencies = package_json["dependencies"]
      .as_object()
      .map(|deps| deps.keys().map(String::from).collect())
      .unwrap_or_default();

    let commands = package_json["scripts"]
      .as_object()
      .map(|scripts| scripts.keys().map(String::from).collect())
      .unwrap_or_default();

    (dependencies, commands)
  }

  fn discover_projects(&self) -> Vec<Project> {
    fs::read_dir(&self.base_path)
      .into_iter()
      .flat_map(|entries| entries.filter_map(Result::ok))
      .filter_map(|entry| {
        let path = entry.path();
        if path.is_dir() {
          let package_json_path = path.join("package.json");
          if let Some(package_json_content) = Self::read_file(&package_json_path) {
            let (dependencies, commands) = Self::parse_package_json(&package_json_content);
            let status = Self::check_running(&path.join("webpack.sock"));

            return Some(Project::new(
              path.file_name()?.to_str()?.to_string(),
              "Description of the project".to_string(),
              dependencies,
              commands,
              status,
            ));
          }
        }
        None
      })
      .collect()
  }

  pub fn get_projects(&self) -> Vec<Project> {
    self.discover_projects()
  }
}
