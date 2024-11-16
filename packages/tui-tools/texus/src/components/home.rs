use color_eyre::Result;

use ratatui::{
  prelude::*,
  widgets::{Block, Borders, List, ListItem},
  crossterm::event::{KeyEvent, KeyCode}
};
use tokio::sync::mpsc::UnboundedSender;

use super::Component;
use crate::{
  action::{Action, ProjectAction},
  config::Config,
  projects::{get_projects, Project},
};

use ratatui::widgets::{Paragraph, Wrap};

#[derive(Default)]
pub struct Home {
  command_tx: Option<UnboundedSender<Action>>,
  config: Config,
  projects: Vec<Project>,
  selected_project_index: usize,
  selected_project: Option<Project>,
}

impl Home {
  pub fn new() -> Self {
    Self {
      projects: get_projects(),
      ..Default::default()
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
      Action::Tick => {
        // add any logic here that should run on every tick
      }
      Action::Render => {
        // add any logic here that should run on every render
      }
      Action::ProjectAction(project_action) => {
        // Handle the specific project actions (Start, Stop, Build)
      }
      _ => {}
    }
    Ok(None)
  }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<Option<Action>> {
    match key {
      KeyEvent {
        code: KeyCode::Char('j') | KeyCode::Down,
        ..
      } => {
        if self.selected_project_index < self.projects.len() - 1 {
          self.selected_project_index += 1;
        }
      }
      KeyEvent {
        code: KeyCode::Char('k') | KeyCode::Up,
        ..
      } => {
        if self.selected_project_index > 0 {
          self.selected_project_index -= 1;
        }
      }
      KeyEvent {
        code:
          KeyCode::Enter
          | KeyCode::Char(' ')
          | KeyCode::Right
          | KeyCode::Char('l'),
        ..
      } => {
        self.selected_project = self.projects.get(self.selected_project_index).cloned();
        // Trigger the ProjectSelect action with the selected project index
        return Ok(Some(Action::ProjectSelect(self.selected_project_index)));
      }
      _ => {}
    }
    Ok(None)
  }

  fn draw(&mut self, frame: &mut Frame, _area: Rect) -> Result<()> {
    let chunks = Layout::horizontal([
      Constraint::Percentage(30), // Left Side (%30)
      Constraint::Percentage(70), // Right Side (%70)
    ])
    .split(frame.area());

    // let selected_project = self.projects.get(selected_index);
    let selected_project = self.projects.get(self.selected_project_index).unwrap();

    let project_items: Vec<ListItem> = self
      .projects
      .iter()
      .enumerate()
      .map(|(i, p)| {
        let mut item = ListItem::new(p.name.clone());
        if i == self.selected_project_index {
          item = item.style(
            Style::default()
              .fg(Color::Yellow)
              .add_modifier(Modifier::BOLD),
          );
        }
        item
      })
      .collect();

    let project_list = List::new(project_items)
      .block(Block::default().title("Projects").borders(Borders::ALL))
      .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    frame.render_widget(project_list, chunks[0]);

    let details_text = serde_json::to_string_pretty(&selected_project).unwrap();

    let project_details = Paragraph::new(details_text)
      .block(Block::default().title("Details").borders(Borders::ALL))
      .wrap(Wrap { trim: false });

    frame.render_widget(project_details, chunks[1]);

    Ok(())
  }
}
