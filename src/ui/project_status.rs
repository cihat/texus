use super::UI;
use crate::components::home::AppState;
use ratatui::{
  prelude::*,
  widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarState, Wrap},
};

pub struct ProjectStatus;

impl UI for ProjectStatus {
  fn draw(state: &AppState, frame: &mut Frame, area: Rect) {
    if let Some(project) = state.get_selected_project() {
      let project_status = Paragraph::new(format!("Status: {}", project.status))
        .block(Block::default().title("Status").borders(Borders::ALL))
        .wrap(Wrap { trim: false });

      let output_block = Block::default().title("Output").borders(Borders::ALL);
      let output_content = project.output.lock().unwrap().clone();
      let total_lines = output_content.lines().count();

      let project_output = Paragraph::new(output_content)
        .block(output_block)
        .wrap(Wrap { trim: false })
        .scroll((state.log_scroll as u16, 0)); // Add vertical scrolling

      let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
        .split(area);

      let mut scrollbar_state = ScrollbarState::new(total_lines).position(state.log_scroll);

      let scrollbar = Scrollbar::default().style(Style::default().fg(Color::Rgb(255, 97, 0)));

      frame.render_widget(project_status, chunks[0]);
      frame.render_widget(project_output, chunks[1]);
      frame.render_stateful_widget(scrollbar, chunks[1], &mut scrollbar_state);
    } else {
      let no_project = Paragraph::new("No project selected.")
        .block(Block::default().title("Status").borders(Borders::ALL))
        .wrap(Wrap { trim: false });
      frame.render_widget(no_project, area);
    }
  }
}
