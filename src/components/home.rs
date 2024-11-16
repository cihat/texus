use super::{logo::Logo, Component};
use crate::{
  action::{Action, ProjectCommand, ProjectScript::*},
  config::Config,
  project_manager::{Project, ProjectManager},
  ui::{
    project_detail::ProjectDetail, project_list::ProjectList, project_status::ProjectStatus, UI,
  },
};
use color_eyre::Result;
use ratatui::{
  crossterm::event::{KeyCode, KeyEvent},
  prelude::*,
  widgets::{Block, Borders},
};
use std::sync::{mpsc::Receiver, Arc};
use strum::Display;
use tokio::sync::mpsc::UnboundedSender;

#[derive(Default, PartialEq, Display, Debug)]
pub enum Mode {
  #[default]
  Normal,
  Search,
}

#[derive(Default, PartialEq, Clone, Display, Debug)]
pub enum ActiveComponent {
  #[default]
  List,
  Detail,
  Status,
}

#[derive(Default, Debug)]
pub struct AppState {
  pub projects: Vec<Project>,        // Owned data
  pub filtered_projects: Vec<usize>, // Indices to the projects vector
  pub selected_project_index: usize,
  pub search_query: String,
  pub mode: Mode,
  pub active_component: ActiveComponent,
  pub detail_scroll: usize,
  pub log_scroll: usize,
  pub logo: Logo,
}

impl AppState {
  pub fn get_selected_project(&self) -> Option<&Project> {
    if let Some(&project_index) = self.filtered_projects.get(self.selected_project_index) {
      self.projects.get(project_index)
    } else {
      None
    }
  }

  pub fn update_filtered_projects(&mut self) {
    self.filtered_projects = self
      .projects
      .iter()
      .enumerate()
      .filter(|(_, project)| {
        // Example filtering logic
        project.name.contains(&self.search_query)
      })
      .map(|(index, _)| index)
      .collect();
    self.selected_project_index = 0;
  }

  pub fn navigate(&mut self, direction: i32) {
    match self.active_component {
      ActiveComponent::List => {
        let len = self.filtered_projects.len();
        if direction > 0 && self.selected_project_index < len - 1 {
          self.selected_project_index += 1;
        } else if direction < 0 && self.selected_project_index > 0 {
          self.selected_project_index -= 1;
        }
      }
      ActiveComponent::Detail => {
        self.detail_scroll = (self.detail_scroll as i32 + direction).max(0) as usize;
      }
      ActiveComponent::Status => {
        tracing::info!("log_scroll: {}", self.log_scroll);
        self.log_scroll = (self.log_scroll as i32 + direction).max(0) as usize;
      } // _ => {}
    }
  }

  pub fn switch_active_component(&mut self, next: bool) {
    self.active_component = match (self.active_component.clone(), next) {
      (ActiveComponent::List, true) => ActiveComponent::Detail,
      (ActiveComponent::Detail, true) => ActiveComponent::Status,
      (ActiveComponent::Status, true) => ActiveComponent::List,
      (ActiveComponent::List, false) => ActiveComponent::Status,
      (ActiveComponent::Detail, false) => ActiveComponent::List,
      (ActiveComponent::Status, false) => ActiveComponent::Detail,
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
  manager: ProjectManager,
}

impl Home {
  pub fn default() -> Self {
    let mut state = AppState::default();
    let (manager, projects) = Self::initialize();

    state.projects = projects;
    state.update_filtered_projects();

    Self {
      state,
      command_tx: None,
      config: Default::default(),
      manager,
    }
  }

  fn initialize() -> (ProjectManager, Vec<Project>) {
    let manager = ProjectManager::default();
    let projects = manager.get_projects();
    (manager, projects)
  }

  fn draw_block(&self, frame: &mut Frame, rect: Rect, title: &str, component: ActiveComponent) {
    let active_border_style = Style::default().fg(Color::Rgb(126, 193, 14));
    let inactive_border_style = Style::default().fg(Color::White);

    let block = Block::default()
      .borders(Borders::ALL)
      .border_style(if self.state.active_component == component {
        active_border_style
      } else {
        inactive_border_style
      })
      .title(title);

    frame.render_widget(block, rect);
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
      Action::ProjectScript(cmd) => {
        if let Some(&project_index) = self
          .state
          .filtered_projects
          .get(self.state.selected_project_index)
        {
          if let Some(project) = self.state.projects.get_mut(project_index) {
            let rx: Receiver<String> = self.manager.execute_script(project, &cmd);
            let project_output = Arc::clone(&project.output);

            // Handle output in a separate thread
            std::thread::spawn(move || {
              while let Ok(message) = rx.recv() {
                let mut output = project_output.lock().unwrap();
                output.push_str(&message); // Append to the vector
              }
            });
          }
        }
      }
      Action::ProjectCommand(cmd) => {
        if let Some(&project_index) = self
          .state
          .filtered_projects
          .get(self.state.selected_project_index)
        {
          if let Some(project) = self.state.projects.get_mut(project_index) {
            self.manager.execute_command(project, &cmd);
          }
        }
      }
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
        KeyCode::Char('s') => return Ok(Some(Action::ProjectScript(Start))),
        KeyCode::Char('b') => return Ok(Some(Action::ProjectScript(Build))),
        //TODO: Implement close and close all project
        KeyCode::Char('c') => return Ok(Some(Action::ProjectCommand(ProjectCommand::Stop))),
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
      let area = frame.area();
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
          Constraint::Percentage(30),
          Constraint::Percentage(45),
        ]
        .as_ref(),
      )
      .split(area);

    // Draw ProjectList
    self.draw_block(frame, rects[0], "Project List", ActiveComponent::List);

    // Draw ProjectDetail
    self.draw_block(frame, rects[1], "Project Detail", ActiveComponent::Detail);

    // Draw ProjectStatus
    self.draw_block(frame, rects[2], "Project Status", ActiveComponent::Status);

    // tracing::info!(
    //   "cihat cihats: {:?}",
    //   self.state.get_selected_project().output
    // );
    ProjectList::draw(&self.state, frame, rects[0]);
    ProjectDetail::draw(&self.state, frame, rects[1]);
    ProjectStatus::draw(&self.state, frame, rects[2]);
    Ok(())
  }
}
