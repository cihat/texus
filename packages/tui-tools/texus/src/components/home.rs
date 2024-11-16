use color_eyre::Result;

use ratatui::{
  crossterm::event::{KeyCode, KeyEvent},
  prelude::*,
  widgets::{Block, Borders},
};
use strum::Display;
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

#[derive(Default, PartialEq, Display)]
pub enum Mode {
  #[default]
  Normal,
  Search,
}

#[derive(Default, PartialEq)]
pub enum ActiveComponent {
  #[default]
  ProjectList,
  ProjectDetail,
  ProjectStatus,
}

#[derive(Default)]
pub struct AppState {
  pub projects: Vec<Project>,
  pub filtered_projects: Vec<Project>,
  pub selected_project_index: usize,
  pub search_query: String,
  pub mode: Mode,
  pub active_component: ActiveComponent,
  pub detail_scroll: usize,
}

impl AppState {
  pub fn get_selected_project(&self) -> Option<&Project> {
    self.filtered_projects.get(self.selected_project_index)
  }

  pub fn update_filtered_projects(&mut self) {
    if self.search_query.is_empty() {
      self.filtered_projects = self.projects.clone();
    } else {
      self.filtered_projects = self
        .projects
        .iter()
        .filter(|p| {
          p.name
            .to_lowercase()
            .contains(&self.search_query.to_lowercase())
        })
        .cloned()
        .collect();
    }
    self.selected_project_index = 0;
  }

  pub fn toggle_search_mode(&mut self) {
    self.mode = match self.mode {
      Mode::Normal => Mode::Search,
      Mode::Search => Mode::Normal,
    };
  }
}

pub struct Home {
  command_tx: Option<UnboundedSender<Action>>,
  config: Config,
  state: AppState,
}

impl Home {
  pub fn new() -> Self {
    let projects = get_projects();
    let filtered_projects = projects.clone();

    Self {
      state: AppState {
        projects,
        filtered_projects,
        ..Default::default()
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
      _ => {}
    }
    Ok(None)
  }

  fn handle_key_event(&mut self, key: KeyEvent) -> Result<Option<Action>> {
    match self.state.mode {
      Mode::Normal => match key.code {
        KeyCode::Char('/') => {
          self.state.toggle_search_mode();
        }
        KeyCode::Char('j') | KeyCode::Down => match self.state.active_component {
          ActiveComponent::ProjectList => {
            if self.state.selected_project_index < self.state.filtered_projects.len() - 1 {
              self.state.selected_project_index += 1;
            }
          }
          ActiveComponent::ProjectDetail => {
            self.state.detail_scroll += 1;
          }
          _ => {}
        },
        KeyCode::Char('k') | KeyCode::Up => match self.state.active_component {
          ActiveComponent::ProjectList => {
            if self.state.selected_project_index > 0 {
              self.state.selected_project_index -= 1;
            }
          }
          ActiveComponent::ProjectDetail => {
            if self.state.detail_scroll > 0 {
              self.state.detail_scroll -= 1;
            }
          }
          _ => {}
        },
        KeyCode::Right | KeyCode::Char('l') => {
          self.state.active_component = match self.state.active_component {
            ActiveComponent::ProjectList => ActiveComponent::ProjectDetail,
            ActiveComponent::ProjectDetail => ActiveComponent::ProjectStatus,
            ActiveComponent::ProjectStatus => ActiveComponent::ProjectList,
          };
          self.state.detail_scroll = 0;
        }
        KeyCode::Left | KeyCode::Char('h') => {
          self.state.active_component = match self.state.active_component {
            ActiveComponent::ProjectList => ActiveComponent::ProjectStatus,
            ActiveComponent::ProjectDetail => ActiveComponent::ProjectList,
            ActiveComponent::ProjectStatus => ActiveComponent::ProjectDetail,
          };
          self.state.detail_scroll = 0;
        }
        _ => {}
      },
      Mode::Search => match key.code {
        KeyCode::Esc => self.state.toggle_search_mode(),
        KeyCode::Char(c) => {
          self.state.search_query.push(c);
          self.state.update_filtered_projects();
        }
        KeyCode::Backspace => {
          self.state.search_query.pop();
          self.state.update_filtered_projects();
        }
        _ => {}
      },
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

    // Define styles for active and inactive borders
    let active_style = Style::default().fg(Color::Yellow);
    let inactive_style = Style::default().fg(Color::White);

    // Draw ProjectList
    let list_block = Block::default()
      .borders(Borders::ALL)
      .style(
        if self.state.active_component == ActiveComponent::ProjectList {
          active_style
        } else {
          inactive_style
        },
      )
      .title("Project List");
    frame.render_widget(list_block, rects[0]);
    ProjectList::draw(&self.state, frame, rects[0]);

    // Draw ProjectDetail
    let detail_block = Block::default()
      .borders(Borders::ALL)
      .style(
        if self.state.active_component == ActiveComponent::ProjectDetail {
          active_style
        } else {
          inactive_style
        },
      )
      .title("Project Detail");
    frame.render_widget(detail_block, rects[1]);
    ProjectDetail::draw(&self.state, frame, rects[1]);

    // Draw ProjectStatus
    let status_block = Block::default()
      .borders(Borders::ALL)
      .style(
        if self.state.active_component == ActiveComponent::ProjectStatus {
          active_style
        } else {
          inactive_style
        },
      )
      .title("Project Status");
    frame.render_widget(status_block, rects[2]);
    ProjectStatus::draw(&self.state, frame, rects[2]);

    Ok(())
  }
}
