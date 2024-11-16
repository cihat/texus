use color_eyre::Result;

use ratatui::{
  crossterm::event::{KeyCode, KeyEvent},
  prelude::*,
  widgets::{block::title, Block, Borders, List, ListItem, Padding, Scrollbar, ScrollbarState},
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
        code: KeyCode::Enter | KeyCode::Char(' ') | KeyCode::Right | KeyCode::Char('l'),
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

  fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
    let rects = Layout::default()
      .direction(Direction::Horizontal)
      .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
      .split(area);

    let list_height = rects[0].height as usize - 2; // Adjust for padding and borders
    let start = self.selected_project_index.saturating_sub(list_height / 2);
    let end = (start + list_height).min(self.projects.len());
    let visible_projects = &self.projects[start..end];

    let project_items: Vec<ListItem> = visible_projects
      .iter()
      .enumerate()
      .map(|(i, p)| {
        let global_index = start + i;
        let mut item = ListItem::new(p.name.clone());
        if global_index == self.selected_project_index {
          item = item.style(
            Style::default()
              .fg(Color::Blue)
              .add_modifier(Modifier::BOLD)
              .add_modifier(Modifier::REVERSED),
          );
        }
        item
      })
      .collect();

    let project_list = List::new(project_items)
      .block(
        Block::default()
          .title(Line::from("Projects").left_aligned())
          .title(Line::from("Middle Title").centered())
          .title(Line::from("Right Title").right_aligned())
          .title_bottom(Line::from("Search: /").left_aligned())
          .padding(Padding::proportional(1))
          .borders(Borders::ALL),
      )
      .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    let selected_project = self.projects.get(self.selected_project_index).unwrap();
    let details_text = serde_json::to_string_pretty(&selected_project).unwrap();
    let project_details = Paragraph::new(details_text)
      .block(
        Block::default()
          .title("Details")
          .style(style::Style::default().fg(Color::Black)) 
          .borders(Borders::ALL)
          .title_bottom(Line::from("Start: s").left_aligned().style(style::Style::default().fg(Color::Green))) 
          .title_bottom(Line::from("Stop: S").centered().style(Style::default().fg(Color::Red)))
          .title_bottom(Line::from("Build: b").right_aligned().style(style::Style::default().fg(Color::Blue))),
      )
      .wrap(Wrap { trim: false });

    let scrollbar = Scrollbar::default().style(Style::default().fg(Color::Green));
    let mut scrollbar_state =
      ScrollbarState::new(self.projects.len()).position(self.selected_project_index);

    frame.render_widget(project_list, rects[0]);
    frame.render_widget(project_details, rects[1]);
    frame.render_stateful_widget(scrollbar, rects[0], &mut scrollbar_state);

    Ok(())
  }
}
