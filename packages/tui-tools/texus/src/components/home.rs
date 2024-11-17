use color_eyre::Result;
use std::path::PathBuf;

use ratatui::{
  crossterm::event::{KeyCode, KeyEvent},
  prelude::*,
  widgets::{Block, Borders},
};
use strum::Display;
use tokio::sync::mpsc::UnboundedSender;

use super::{
  logo::Logo, project_detail::ProjectDetail, project_list::ProjectList,
  project_status::ProjectStatus, Component,
};
use crate::{
  action::Action,
  config::Config,
  project::{Project, ProjectManager},
};

#[derive(Default, PartialEq, Display)]
pub enum Mode {
  #[default]
  Normal,
  Search,
}

#[derive(Default, PartialEq, Clone)]
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
  pub logo: Logo,
}

impl AppState {
  pub fn get_selected_project(&self) -> Project {
    self
      .filtered_projects
      .get(self.selected_project_index)
      .cloned()
      .unwrap_or_default()
  }

  pub fn update_filtered_projects(&mut self) {
    self.filtered_projects = if self.search_query.is_empty() {
      self.projects.clone()
    } else {
      self
        .projects
        .iter()
        .filter(|p| {
          p.name
            .to_lowercase()
            .contains(&self.search_query.to_lowercase())
        })
        .cloned()
        .collect()
    };
    self.selected_project_index = 0;
  }

  pub fn navigate(&mut self, direction: i32) {
    match self.active_component {
      ActiveComponent::ProjectList => {
        let len = self.filtered_projects.len();
        if direction > 0 && self.selected_project_index < len - 1 {
          self.selected_project_index += 1;
        } else if direction < 0 && self.selected_project_index > 0 {
          self.selected_project_index -= 1;
        }
      }
      ActiveComponent::ProjectDetail => {
        self.detail_scroll = (self.detail_scroll as i32 + direction).max(0) as usize;
      }
      _ => {}
    }
  }

  pub fn switch_active_component(&mut self, next: bool) {
    self.active_component = match (self.active_component.clone(), next) {
      (ActiveComponent::ProjectList, true) => ActiveComponent::ProjectDetail,
      (ActiveComponent::ProjectDetail, true) => ActiveComponent::ProjectStatus,
      (ActiveComponent::ProjectStatus, true) => ActiveComponent::ProjectList,
      (ActiveComponent::ProjectList, false) => ActiveComponent::ProjectStatus,
      (ActiveComponent::ProjectDetail, false) => ActiveComponent::ProjectList,
      (ActiveComponent::ProjectStatus, false) => ActiveComponent::ProjectDetail,
    };
    self.detail_scroll = 0;
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
    let mut state = AppState::default();
    state.projects = Self::initialize_projects();
    state.filtered_projects = state.projects.clone();
    Self {
      state,
      command_tx: None,
      config: Default::default(),
    }
  }

  fn initialize_projects() -> Vec<Project> {
    // let project_manager = ProjectManager::new(PathBuf::from(
    //   "/Users/cihatsalik/www/frontend/packages/apps",
    // ));
    let project_manager = ProjectManager::default();
    project_manager.get_projects()
  }

  fn create_block(title: &str, active: bool) -> Block {
    Block::default()
      .borders(Borders::ALL)
      .style(if active {
        Style::default().fg(Color::Rgb(126, 193, 14))
      } else {
        Style::default().fg(Color::White)
      })
      .title(title)
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
      Action::Tick => { /* Handle periodic updates */ }
      Action::Render => { /* Handle rendering logic */ }
      _ => {}
    }
    Ok(None)
  }

  fn handle_key_event(&mut self, key: KeyEvent) -> Result<Option<Action>> {
    match self.state.mode {
      Mode::Normal => match key.code {
        KeyCode::Char('/') => self.state.toggle_search_mode(),
        KeyCode::Char('j') | KeyCode::Down => self.state.navigate(1),
        KeyCode::Char('k') | KeyCode::Up => self.state.navigate(-1),
        KeyCode::Char('l') | KeyCode::Right => self.state.switch_active_component(true),
        KeyCode::Char('h') | KeyCode::Left => self.state.switch_active_component(false),
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
    if !self.state.logo.is_rendered && !self.state.projects.is_empty() {
      let (logo_width, logo_height) = self.state.logo.get_size();
      if logo_width < area.width && logo_height < area.height {
        frame.render_widget(
          &self.state.logo,
          Rect::new(
            area.width / 2 - logo_width / 2,
            area.height / 2 - logo_height / 2,
            logo_width,
            logo_height,
          ),
        );
        self.state.logo.is_rendered = self.state.logo.init_time.elapsed().as_millis() > 500;
      }
    }
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
    let active_style = Style::default().fg(Color::Rgb(126, 193, 14));
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
