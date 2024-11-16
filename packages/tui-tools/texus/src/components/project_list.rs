use ratatui::{
  prelude::*,
  widgets::{Block, Borders, List, ListItem, Scrollbar, ScrollbarState},
};

use super::home::AppState;

pub struct ProjectList;

impl ProjectList {
  pub fn draw(state: &AppState, frame: &mut Frame, area: Rect) {
    let list_height = area.height as usize - 2; // Adjust for padding and borders
    let start = state.selected_project_index.saturating_sub(list_height / 2);
    let end = (start + list_height).min(state.projects.len());
    let visible_projects = &state.projects[start..end];

    let project_items: Vec<ListItem> = visible_projects
      .iter()
      .enumerate()
      .map(|(i, project)| {
        let global_index = start + i;
        let mut item = ListItem::new(project.name.clone());
        if global_index == state.selected_project_index {
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
          .title_bottom(Line::from("Search: /").left_aligned().bold())
          .borders(Borders::ALL),
      )
      .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    let scrollbar = Scrollbar::default().style(Style::default().fg(Color::Green));
    let mut scrollbar_state =
      ScrollbarState::new(state.projects.len()).position(state.selected_project_index);

    frame.render_widget(project_list, area);
    frame.render_stateful_widget(scrollbar, area, &mut scrollbar_state);
  }
}
