use ratatui::{layout::Rect, Frame};

use crate::components::home::AppState;

pub mod project_detail;
pub mod project_list;
pub mod project_status;

pub trait UI {
  fn draw(state: &AppState, frame: &mut Frame, area: Rect);
}
