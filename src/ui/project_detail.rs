use ratatui::{
  prelude::*,
  widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarState, Wrap},
};

use crate::components::home::AppState;

use super::UI;

pub struct ProjectDetail;

impl ProjectDetail {
  fn build_block<'a>() -> Block<'a> {
    let start = Line::from("Start: s").style(Style::default().fg(Color::Green).bold());
    let kill = Line::from("| Kill: c").style(Style::default().fg(Color::Red).bold());
    let build = Line::from("| Build: b").style(Style::default().fg(Color::Blue).bold());

    Block::default()
      .title("Details")
      .borders(Borders::ALL)
      .title_bottom(start)
      .title_bottom(kill)
      .title_bottom(build)
  }
}

impl UI for ProjectDetail {
  fn draw(state: &AppState, frame: &mut Frame, area: Rect) {
    let project = state.get_selected_project();
    let details_text = serde_json::to_string_pretty(&project)
      .unwrap_or_else(|_| "Error formatting project details".to_string());
    let lines: Vec<&str> = details_text.lines().collect();
    let visible_lines = lines
      .iter()
      .skip(state.detail_scroll)
      .take(area.height as usize)
      .cloned()
      .collect::<Vec<&str>>()
      .join("\n");
    let project_details = Paragraph::new(visible_lines)
      .block(Block::default().title("Details").borders(Borders::ALL))
      .block(Self::build_block())
      .wrap(Wrap { trim: false });
    let scrollbar = Scrollbar::default().style(Style::default().fg(Color::Rgb(255, 97, 0)));
    let mut scrollbar_state =
      ScrollbarState::new(details_text.lines().count()).position(state.detail_scroll);

    frame.render_widget(project_details, area);
    frame.render_stateful_widget(scrollbar, area, &mut scrollbar_state);
  }
}
