use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader};
use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::{env, fs};
use strum::Display;
use sysinfo::System;

use crate::action::{ProjectCommand, ProjectScript};

#[derive(Default, Serialize, Deserialize, PartialEq, Eq, Clone, Display, Debug, Copy)]
pub enum ProjectStatus {
  #[default]
  // Initialized,
  Running,
  Stopped,
  Completed,
  Error,
  Idle,
}

#[derive(Default, Clone, Debug)]
pub struct Project {
  pub name: String,
  pub status: ProjectStatus,
  pub dependencies: Vec<String>,
  pub commands: Vec<String>,
  pub output: Arc<Mutex<String>>,
  pub pid: Option<u32>,
}

impl Serialize for Project {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    let mut state = serializer.serialize_struct("Project", 5)?;
    state.serialize_field("name", &self.name)?;
    state.serialize_field("status", &self.status)?;
    state.serialize_field("dependencies", &self.dependencies)?;
    state.serialize_field("commands", &self.commands)?;
    state.end()
  }
}

impl Project {
  pub fn new(
    name: String,
    dependencies: Vec<String>,
    commands: Vec<String>,
    status: ProjectStatus,
    output: Arc<Mutex<String>>,
    pid: Option<u32>,
  ) -> Self {
    Self {
      name,
      dependencies,
      commands,
      status,
      output,
      pid,
    }
  }
}

#[derive(Default, Debug)]
pub struct ProjectManager {
  pub base_path: PathBuf,
}

impl ProjectManager {
  pub fn default() -> Self {
    dotenvy::from_path(".env").ok();
    if let Ok(local_path) = env::var("TEXUS_MONOREPO_PATH") {
      Self {
        base_path: PathBuf::from(local_path),
      }
    } else {
      panic!("TEXUS_MONOREPO_PATH variable not found in .env");
    }
  }

  fn read_file(path: &PathBuf) -> Option<String> {
    fs::read_to_string(path).ok()
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

  fn terminate_process(pid: u32) -> Result<(), String> {
    #[cfg(unix)]
    {
      let result = Command::new("kill")
        .arg("-TERM") // Send a SIGTERM signal
        .arg(pid.to_string())
        .status();

      match result {
        Ok(status) if status.success() => Ok(()),
        Ok(_) => Err(format!("Failed to terminate process {}", pid)),
        Err(err) => Err(format!("Error terminating process {}: {}", pid, err)),
      }
    }
  }

  //? Refactor
  pub fn is_running(&self, project: &Project) -> bool {
    let mut sys = System::new_all();
    let mut is_run = false;
    sys.refresh_all();

    if let Some(pid) = project.pid {
      is_run = sys.process(sysinfo::Pid::from(pid as usize)).is_some();
    }

    if project.pid.is_some() && is_run {
      true
    } else {
      false
    }
  }

  //? REFACTOR: this all function is doing too much and we can change with third party library for better performance, readability and multiplatform support
  ///! and fix it: We can't close after running at the moment
  pub fn check_running(path: &PathBuf) -> (ProjectStatus, Option<u32>) {
    let sock_path = path.join("webpack.sock");

    if fs::metadata(&sock_path).is_ok() {
      if let Ok(stream) = UnixStream::connect(&sock_path) {
        drop(stream);

        // Use ps to find node process in the directory
        if let Ok(output) = Command::new("ps").args(["aux"]).output() {
          let output_str = String::from_utf8_lossy(&output.stdout);
          let path_str = path.to_str().unwrap_or_default();

          if let Some(line) = output_str
            .lines()
            .find(|line| line.contains("node") && line.contains(path_str))
          {
            let pid = line
              .split_whitespace()
              .nth(1)
              .and_then(|pid_str| pid_str.parse::<u32>().ok());

            tracing::info!("Found running process: {:?}", pid);

            return (ProjectStatus::Running, Some(pid.unwrap()));
          }
        }

        return (ProjectStatus::Running, None);
      }
    }
    (ProjectStatus::Idle, None)
  }

  pub fn execute_script(
    &mut self,
    project: &mut Project,
    cmd: &ProjectScript,
  ) -> mpsc::Receiver<String> {
    let project_path = self.base_path.join(&project.name);
    let package_json_path = project_path.join("package.json");

    let (tx, rx) = mpsc::channel();

    let shared_output = Arc::new(Mutex::new(String::new()));
    let output_clone = Arc::clone(&shared_output);
    let package_manager = "pnpm";

    if let Some(content) = Self::read_file(&package_json_path) {
      let (dependencies, commands) = Self::parse_package_json(&content);

      if !commands.contains(&cmd.to_string()) {
        let error_msg = format!("No '{}' command found in package.json", cmd);
        tx.send(error_msg.clone()).unwrap();
        let mut output = output_clone.lock().unwrap();
        output.push_str(&error_msg);
        output.push('\n');

        return rx;
      }

      project.dependencies = dependencies;
      project.commands = commands;
    } else {
      let error_msg = "Failed to read package.json".to_string();
      tx.send(error_msg.clone()).unwrap();
      let mut output = output_clone.lock().unwrap();
      output.push_str(&error_msg);
      output.push('\n');

      return rx;
    }

    let project_name = project.name.clone();
    project.status = ProjectStatus::Running;

    let command_type = cmd.clone();
    thread::spawn(move || {
      let initial_msg = format!("Attempting to start project: {}", project_name);
      tx.send(initial_msg.clone()).unwrap();

      {
        let mut output = output_clone.lock().unwrap();
        output.push_str(&initial_msg);
        output.push('\n');
      }

      let mut command = Command::new(package_manager);
      command
        .current_dir(&project_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

      match command_type {
        ProjectScript::Start => {
          command.arg("start");
        }
        ProjectScript::Build => {
          command.arg("build");
        }
      }

      match command.spawn() {
        Ok(mut child) => {
          if let Some(stdout) = child.stdout.take() {
            let reader = BufReader::new(stdout);
            for line in reader.lines().map_while(Result::ok) {
              tx.send(line.clone()).unwrap();
              let mut output = output_clone.lock().unwrap();
              output.push_str(&line);
              output.push('\n');
            }
          }

          if let Some(stderr) = child.stderr.take() {
            let reader = BufReader::new(stderr);
            for line in reader.lines().map_while(Result::ok) {
              tx.send(line.clone()).unwrap();

              let mut output = output_clone.lock().unwrap();
              output.push_str(&line);
              output.push('\n');
            }
          }

          match child.wait() {
            Ok(status) => {
              let status_msg = format!("Project {} finished with status: {}", project_name, status);
              tx.send(status_msg.clone()).unwrap();

              let mut output = output_clone.lock().unwrap();
              output.push_str(&status_msg);
              output.push('\n');
            }
            Err(e) => {
              let error_msg = format!("Error waiting for project {}: {}", project_name, e);
              tx.send(error_msg.clone()).unwrap();

              let mut output = output_clone.lock().unwrap();
              output.push_str(&error_msg);
              output.push('\n');
            }
          }
        }
        Err(e) => {
          let error_msg = format!("Failed to start project {}: {}", project_name, e);
          tx.send(error_msg.clone()).unwrap();

          let mut output = output_clone.lock().unwrap();
          output.push_str(&error_msg);
          output.push('\n');
        }
      }
    });

    thread::spawn(move || loop {
      std::thread::sleep(std::time::Duration::from_millis(100));
    });

    rx
  }

  pub fn execute_command(&self, project: &mut Project, cmd: &ProjectCommand) {
    let project_path = self.base_path.join(&project.name);
    Self::check_running(&project_path);

    match cmd {
      ProjectCommand::Stop => {
        if self.is_running(project) {
          if let Some(pid) = project.pid {
            if let Err(err) = Self::terminate_process(pid) {
              let mut output = project.output.lock().unwrap();
              *output = format!("Failed to stop project {}: {}", project.name, err);
            } else {
              project.status = ProjectStatus::Stopped;
              project.pid = None;
              let mut output = project.output.lock().unwrap();
              *output = format!("Project {} stopped successfully.", project.name);
            }
          }
        } else {
          let mut output = project.output.lock().unwrap();
          *output = format!("Project {} is not running.", project.name);
        }
      }
      ProjectCommand::StopAll => {
        // Stop all projects
      }
    }
  }

  pub fn get_projects(&self) -> Vec<Project> {
    fs::read_dir(&self.base_path)
      .into_iter()
      .flat_map(|entries| entries.filter_map(Result::ok))
      .filter_map(|entry| {
        let path = entry.path();
        if path.is_dir() {
          let package_json_path = path.join("package.json");
          if let Some(package_json_content) = Self::read_file(&package_json_path) {
            let (dependencies, commands) = Self::parse_package_json(&package_json_content);
            let (status, pid) = Self::check_running(&path);

            tracing::info!("found pid {:?}", pid);

            return Some(Project::new(
              path.file_name()?.to_str()?.to_string(),
              dependencies,
              commands,
              status,
              Arc::new(Mutex::new(String::new())),
              pid,
            ));
          }
        }
        None
      })
      .collect()
  }
}
