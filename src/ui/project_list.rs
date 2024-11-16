use crate::{components::home::Mode, project_manager};
use ratatui::{
  prelude::*,
  widgets::{Block, Borders, List, ListItem, Scrollbar, ScrollbarState},
};

use project_manager::ProjectStatus::Running;

use crate::components::home::AppState;

use super::UI;

pub struct ProjectList;

impl UI for ProjectList {
  fn draw(state: &AppState, frame: &mut Frame, area: Rect) {
    let list_height = area.height as usize - 2;

    let start = state.selected_project_index.saturating_sub(list_height / 2);
    let end = (start + list_height).min(state.filtered_projects.len());

    let visible_projects = if start < state.filtered_projects.len() {
      &state.filtered_projects[start..end]
    } else {
      &[]
    };

    let project_items: Vec<ListItem> = visible_projects
      .iter()
      .enumerate()
      .map(|(i, &project_index)| {
        let project = &state.projects[project_index];
        let global_index = start + i;
        let text_color = if project.status == Running {
          Color::Rgb(126, 193, 14)
        } else {
          Color::White
        };
        let mut item = ListItem::new(format!(
          "{}{}",
          if project.status == Running {
            "● "
          } else {
            "○ "
          },
          project.name
        ))
        .style(Style::default().fg(text_color));

        if global_index == state.selected_project_index {
          item = item.style(
            Style::default()
              .fg(Color::Rgb(0, 163, 225))
              .bg(Color::LightYellow)
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
          .title(Line::from("Frontend Projects").left_aligned())
          .title_bottom(if state.mode == Mode::Search {
            Line::from(format!("/ {}", state.search_query))
          } else {
            Line::raw("")
          })
          .title_bottom(
            Line::from(format!("Mode: {}", state.mode))
              .right_aligned()
              .bold(),
          )
          .title_bottom(Line::from("←↓↑→/hjkl to navigate, / to search"))
          .borders(Borders::ALL),
      )
      .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    let scrollbar = Scrollbar::default().style(Style::default().fg(Color::Rgb(255, 97, 0)));
    let mut scrollbar_state =
      ScrollbarState::new(state.filtered_projects.len()).position(state.selected_project_index);

    frame.render_widget(project_list, area);
    frame.render_stateful_widget(scrollbar, area, &mut scrollbar_state);
  }
}
