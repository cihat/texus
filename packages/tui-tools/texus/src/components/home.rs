use color_eyre::Result;

use ratatui::{
  crossterm::event::{KeyCode, KeyEvent},
  prelude::*,
};
use tokio::sync::mpsc::UnboundedSender;

use super::{
  project_detail::ProjectDetail, project_list::ProjectList, project_status::ProjectStatus,
  Component,
};
use crate::{
  action::Action,
  config::Config,
  projects::{get_projects, Project},
};

pub struct AppState {
  pub projects: Vec<Project>,
  pub selected_project_index: usize,
}

impl AppState {
  pub fn selected_project(&self) -> Option<&Project> {
    self.projects.get(self.selected_project_index)
  }
}

pub struct Home {
  command_tx: Option<UnboundedSender<Action>>,
  config: Config,
  state: AppState,
}

impl Home {
  pub fn new() -> Self {
    Self {
      state: AppState {
        projects: get_projects(),
        selected_project_index: 0,
      },
      command_tx: None,
      config: Default::default(),
    }
  }
}

impl Component for Home {
  fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
    self.command_tx = Some(tx);
    Ok(())
  }

  fn register_config_handler(&mut self, config: Config) -> Result<()> {
    self.config = config;
    Ok(())
  }

  fn update(&mut self, action: Action) -> Result<Option<Action>> {
    match action {
      Action::Tick => { /* Handle tick */ }
      Action::Render => { /* Handle render */ }
      Action::ProjectAction(_project_action) => { /* Handle project actions */ }
      _ => {}
    }
    Ok(None)
  }

  fn handle_key_event(&mut self, key: KeyEvent) -> Result<Option<Action>> {
    match key.code {
      KeyCode::Char('j') | KeyCode::Down => {
        if self.state.selected_project_index < self.state.projects.len() - 1 {
          self.state.selected_project_index += 1;
        }
      }
      KeyCode::Char('k') | KeyCode::Up => {
        if self.state.selected_project_index > 0 {
          self.state.selected_project_index -= 1;
        }
      }
      KeyCode::Enter | KeyCode::Right => {
        // Set selected project
        return Ok(Some(Action::ProjectSelect(
          self.state.selected_project_index,
        )));
      }
      _ => {}
    }
    Ok(None)
  }

  fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
    let rects = Layout::default()
      .direction(Direction::Horizontal)
      .constraints(
        [
          Constraint::Percentage(25),
          Constraint::Percentage(40),
          Constraint::Percentage(35),
        ]
        .as_ref(),
      )
      .split(area);

    ProjectList::draw(&self.state, frame, rects[0]);
    ProjectDetail::draw(&self.state, frame, rects[1]);
    ProjectStatus::draw(&self.state, frame, rects[2]);

    Ok(())
  }
}
