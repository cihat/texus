use ratatui::{
  prelude::*,
  widgets::{Block, Borders, Paragraph, Wrap},
};

use super::home::AppState;

pub struct ProjectDetail;

impl ProjectDetail {
  pub fn draw(state: &AppState, frame: &mut Frame, area: Rect) {
    if let Some(project) = state.selected_project() {
      let details_text = serde_json::to_string_pretty(&project).unwrap();
      let project_details = Paragraph::new(details_text)
        .block(
          Block::default()
            .title("Details")
            .borders(Borders::ALL)
            .title_bottom(
              Line::from("Start: s")
                .left_aligned()
                .bold()
                .style(style::Style::default().fg(Color::Green)),
            )
            .title_bottom(
              Line::from("Stop: S")
                .centered()
                .bold()
                .style(Style::default().fg(Color::Red)),
            )
            .title_bottom(
              Line::from("Build: b")
                .right_aligned()
                .bold()
                .style(style::Style::default().fg(Color::Blue)),
            )
            .borders(Borders::ALL),
        )
        .wrap(Wrap { trim: false });

      frame.render_widget(project_details, area);
    }
  }
}
