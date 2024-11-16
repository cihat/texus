use ratatui::{
  prelude::*,
  widgets::{Block, Borders, Paragraph, Wrap},
};

use super::home::AppState;

pub struct ProjectStatus;

impl ProjectStatus {
  pub fn draw(state: &AppState, frame: &mut Frame, area: Rect) {
    if let Some(project) = state.selected_project() {
      let project_status = Paragraph::new(format!("Status: {}", project.status))
        .block(Block::default().title("Status").borders(Borders::ALL))
        .wrap(Wrap { trim: false });

      frame.render_widget(project_status, area);
    }
  }
}
