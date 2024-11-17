use std::time::Instant;

use ansi_to_tui::IntoText;
use ratatui::{
  buffer::Buffer,
  layout::Rect,
  style::{Color, Style, Stylize},
  text::Text,
  widgets::{Widget, WidgetRef},
};

const LOGO: &str = "
\x1b[49m                                                \x1b[m
\x1b[49m                                                \x1b[m
\x1b[49m                                                \x1b[m
\x1b[49m                                                \x1b[m
\x1b[49m                      \x1b[38;5;15;49m▄\x1b[38;5;221;48;5;15m▄\x1b[38;5;15;49m▄\x1b[49m \x1b[38;5;15;49m▄▄\x1b[49m                    \x1b[m
\x1b[49m      \x1b[38;5;15;49m▄\x1b[38;5;215;48;5;224m▄\x1b[38;5;173;48;5;224m▄▄\x1b[38;5;222;48;5;15m▄\x1b[38;5;15;49m▄\x1b[49m   \x1b[38;5;15;49m▄▄▄\x1b[38;5;222;48;5;15m▄▄▄\x1b[38;5;215;48;5;230m▄\x1b[38;5;214;48;5;214m▄▄\x1b[38;5;214;48;5;222m▄\x1b[38;5;222;48;5;15m▄\x1b[38;5;214;48;5;222m▄▄\x1b[38;5;221;48;5;255m▄\x1b[38;5;222;48;5;15m▄▄\x1b[38;5;230;49m▄\x1b[38;5;15;49m▄▄▄\x1b[49m  \x1b[38;5;15;49m▄\x1b[38;5;216;48;5;15m▄\x1b[38;5;172;48;5;223m▄\x1b[38;5;173;48;5;224m▄\x1b[38;5;216;48;5;15m▄\x1b[38;5;15;49m▄\x1b[49m     \x1b[m
\x1b[49m      \x1b[49;38;5;15m▀\x1b[38;5;223;48;5;173m▄\x1b[38;5;173;48;5;166m▄\x1b[48;5;166m \x1b[38;5;172;48;5;173m▄\x1b[38;5;172;48;5;223m▄\x1b[38;5;223;48;5;15m▄\x1b[38;5;230;49m▄\x1b[38;5;221;48;5;15m▄\x1b[38;5;214;48;5;221m▄\x1b[38;5;214;48;5;215m▄▄\x1b[38;5;221;48;5;214m▄▄▄▄▄▄\x1b[38;5;221;48;5;221m▄▄▄\x1b[38;5;221;48;5;214m▄▄▄▄\x1b[38;5;215;48;5;215m▄\x1b[38;5;214;48;5;215m▄▄\x1b[38;5;214;48;5;230m▄\x1b[38;5;224;49m▄\x1b[38;5;222;48;5;15m▄\x1b[38;5;172;48;5;215m▄\x1b[38;5;166;48;5;166m▄▄\x1b[38;5;172;48;5;167m▄\x1b[38;5;15;48;5;173m▄\x1b[49;38;5;15m▀\x1b[49m     \x1b[m
\x1b[49m       \x1b[38;5;223;48;5;223m▄\x1b[38;5;173;48;5;172m▄\x1b[38;5;214;48;5;208m▄\x1b[38;5;208;48;5;166m▄\x1b[38;5;172;48;5;172m▄\x1b[38;5;214;48;5;173m▄\x1b[38;5;221;48;5;214m▄▄\x1b[38;5;221;48;5;221m▄\x1b[48;5;221m                  \x1b[38;5;221;48;5;221m▄\x1b[38;5;221;48;5;214m▄\x1b[38;5;215;48;5;173m▄\x1b[38;5;172;48;5;167m▄\x1b[38;5;214;48;5;166m▄\x1b[38;5;214;48;5;172m▄\x1b[38;5;173;48;5;173m▄\x1b[38;5;15;48;5;15m▄\x1b[49m      \x1b[m
\x1b[49m       \x1b[49;38;5;15m▀\x1b[38;5;15;48;5;216m▄\x1b[38;5;214;48;5;214m▄▄\x1b[38;5;221;48;5;214m▄\x1b[38;5;221;48;5;221m▄▄▄\x1b[38;5;222;48;5;221m▄\x1b[38;5;223;48;5;221m▄▄▄\x1b[38;5;221;48;5;221m▄\x1b[48;5;221m         \x1b[38;5;221;48;5;221m▄\x1b[38;5;222;48;5;221m▄\x1b[38;5;223;48;5;221m▄▄▄\x1b[38;5;221;48;5;221m▄▄▄\x1b[38;5;221;48;5;215m▄\x1b[38;5;215;48;5;214m▄\x1b[38;5;221;48;5;179m▄\x1b[38;5;223;48;5;223m▄\x1b[38;5;15;49m▄\x1b[49m      \x1b[m
\x1b[49m       \x1b[38;5;15;49m▄\x1b[38;5;221;48;5;15m▄\x1b[38;5;214;48;5;214m▄\x1b[38;5;221;48;5;215m▄\x1b[48;5;221m \x1b[38;5;221;48;5;221m▄\x1b[38;5;222;48;5;221m▄\x1b[38;5;102;48;5;222m▄\x1b[38;5;251;48;5;241m▄\x1b[38;5;238;48;5;239m▄\x1b[38;5;15;48;5;239m▄\x1b[38;5;15;48;5;240m▄\x1b[38;5;237;48;5;230m▄\x1b[38;5;229;48;5;221m▄\x1b[38;5;221;48;5;221m▄\x1b[48;5;221m     \x1b[38;5;221;48;5;221m▄▄\x1b[38;5;252;48;5;221m▄\x1b[38;5;245;48;5;247m▄\x1b[38;5;238;48;5;239m▄\x1b[38;5;15;48;5;239m▄▄\x1b[38;5;239;48;5;187m▄\x1b[38;5;15;48;5;221m▄\x1b[38;5;221;48;5;221m▄\x1b[48;5;221m \x1b[38;5;221;48;5;221m▄▄\x1b[38;5;214;48;5;215m▄\x1b[38;5;230;48;5;15m▄\x1b[38;5;15;49m▄\x1b[49m     \x1b[m
\x1b[49m      \x1b[38;5;15;49m▄\x1b[38;5;222;48;5;223m▄\x1b[38;5;221;48;5;214m▄\x1b[38;5;221;48;5;221m▄\x1b[48;5;221m  \x1b[38;5;221;48;5;221m▄\x1b[38;5;222;48;5;222m▄\x1b[38;5;102;48;5;245m▄\x1b[38;5;248;48;5;234m▄\x1b[38;5;249;48;5;238m▄\x1b[38;5;59;48;5;15m▄▄\x1b[38;5;237;48;5;236m▄\x1b[38;5;229;48;5;229m▄\x1b[48;5;221m       \x1b[38;5;221;48;5;221m▄\x1b[38;5;252;48;5;252m▄\x1b[38;5;243;48;5;234m▄\x1b[38;5;7;48;5;234m▄\x1b[38;5;59;48;5;15m▄▄\x1b[38;5;236;48;5;239m▄\x1b[38;5;15;48;5;15m▄\x1b[48;5;221m    \x1b[38;5;215;48;5;215m▄\x1b[38;5;214;48;5;214m▄\x1b[48;5;15m \x1b[49m     \x1b[m
\x1b[49m      \x1b[38;5;15;48;5;15m▄\x1b[38;5;214;48;5;215m▄\x1b[38;5;221;48;5;221m▄\x1b[48;5;221m   \x1b[38;5;221;48;5;221m▄\x1b[38;5;221;48;5;222m▄\x1b[38;5;222;48;5;245m▄\x1b[38;5;8;48;5;188m▄\x1b[38;5;242;48;5;188m▄\x1b[38;5;242;48;5;234m▄\x1b[38;5;242;48;5;235m▄\x1b[38;5;229;48;5;237m▄\x1b[38;5;221;48;5;229m▄\x1b[48;5;221m  \x1b[38;5;179;48;5;221m▄\x1b[38;5;137;48;5;221m▄▄\x1b[48;5;221m \x1b[38;5;221;48;5;221m▄▄\x1b[38;5;221;48;5;252m▄\x1b[38;5;144;48;5;246m▄\x1b[38;5;242;48;5;15m▄\x1b[38;5;242;48;5;234m▄▄\x1b[38;5;187;48;5;235m▄\x1b[38;5;221;48;5;255m▄\x1b[38;5;221;48;5;221m▄\x1b[48;5;221m  \x1b[38;5;221;48;5;221m▄\x1b[38;5;221;48;5;214m▄\x1b[38;5;214;48;5;214m▄\x1b[38;5;221;48;5;15m▄\x1b[38;5;15;49m▄\x1b[49m    \x1b[m
\x1b[49m      \x1b[48;5;15m \x1b[48;5;214m \x1b[48;5;221m  \x1b[38;5;221;48;5;221m▄\x1b[38;5;215;48;5;221m▄▄▄▄\x1b[38;5;221;48;5;222m▄▄▄▄\x1b[38;5;221;48;5;221m▄\x1b[48;5;221m   \x1b[38;5;221;48;5;137m▄\x1b[38;5;215;48;5;0m▄\x1b[38;5;221;48;5;0m▄\x1b[48;5;221m   \x1b[38;5;221;48;5;221m▄\x1b[38;5;221;48;5;222m▄▄▄▄\x1b[38;5;215;48;5;221m▄▄▄▄\x1b[38;5;221;48;5;221m▄▄\x1b[48;5;221m \x1b[38;5;214;48;5;214m▄\x1b[48;5;221m \x1b[48;5;15m \x1b[49m    \x1b[m
\x1b[49m      \x1b[49;38;5;15m▀\x1b[38;5;223;48;5;215m▄\x1b[38;5;214;48;5;215m▄\x1b[38;5;221;48;5;221m▄▄\x1b[38;5;215;48;5;215m▄▄\x1b[48;5;215m \x1b[38;5;215;48;5;215m▄\x1b[38;5;221;48;5;215m▄\x1b[38;5;221;48;5;221m▄\x1b[48;5;221m \x1b[38;5;221;48;5;221m▄▄▄▄▄▄▄▄▄▄▄▄▄\x1b[48;5;221m \x1b[38;5;221;48;5;221m▄▄\x1b[38;5;215;48;5;215m▄▄▄▄\x1b[38;5;221;48;5;215m▄\x1b[48;5;221m \x1b[38;5;215;48;5;221m▄\x1b[38;5;214;48;5;214m▄\x1b[38;5;15;48;5;222m▄\x1b[49;38;5;15m▀\x1b[49m    \x1b[m
\x1b[49m       \x1b[38;5;223;48;5;223m▄\x1b[38;5;214;48;5;214m▄\x1b[38;5;221;48;5;221m▄\x1b[48;5;221m \x1b[38;5;221;48;5;221m▄▄▄▄\x1b[48;5;221m     \x1b[38;5;221;48;5;221m▄\x1b[38;5;221;48;5;215m▄▄\x1b[38;5;215;48;5;221m▄▄▄▄\x1b[38;5;221;48;5;215m▄\x1b[38;5;221;48;5;221m▄\x1b[48;5;221m    \x1b[38;5;221;48;5;221m▄▄▄▄▄▄▄\x1b[38;5;214;48;5;215m▄\x1b[38;5;221;48;5;214m▄\x1b[38;5;15;48;5;15m▄\x1b[49m     \x1b[m
\x1b[49m       \x1b[49;38;5;15m▀\x1b[38;5;15;48;5;223m▄\x1b[38;5;214;48;5;214m▄\x1b[38;5;215;48;5;221m▄\x1b[38;5;221;48;5;221m▄\x1b[48;5;221m          \x1b[38;5;221;48;5;221m▄▄▄▄▄\x1b[48;5;221m          \x1b[38;5;221;48;5;221m▄▄\x1b[38;5;221;48;5;215m▄\x1b[38;5;255;48;5;221m▄\x1b[49;38;5;15m▀\x1b[49m      \x1b[m
\x1b[49m         \x1b[38;5;15;48;5;15m▄\x1b[38;5;222;48;5;214m▄\x1b[38;5;215;48;5;215m▄\x1b[38;5;215;48;5;221m▄\x1b[38;5;221;48;5;221m▄\x1b[48;5;221m                   \x1b[38;5;221;48;5;221m▄▄▄▄\x1b[38;5;214;48;5;221m▄\x1b[38;5;222;48;5;214m▄\x1b[38;5;15;48;5;15m▄\x1b[49m        \x1b[m
\x1b[49m          \x1b[49;38;5;15m▀\x1b[49;38;5;230m▀\x1b[38;5;15;48;5;215m▄\x1b[38;5;214;48;5;214m▄\x1b[38;5;214;48;5;221m▄\x1b[38;5;221;48;5;221m▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄\x1b[38;5;214;48;5;221m▄\x1b[38;5;215;48;5;214m▄\x1b[38;5;222;48;5;214m▄\x1b[38;5;15;48;5;222m▄\x1b[49;38;5;15m▀\x1b[49m         \x1b[m
\x1b[49m             \x1b[49;38;5;15m▀\x1b[38;5;15;48;5;223m▄\x1b[38;5;229;48;5;214m▄\x1b[38;5;223;48;5;214m▄▄\x1b[38;5;215;48;5;215m▄\x1b[38;5;214;48;5;221m▄▄▄▄▄▄▄▄▄▄▄▄\x1b[38;5;223;48;5;214m▄▄▄\x1b[38;5;230;48;5;221m▄\x1b[49;38;5;15m▀▀\x1b[49m           \x1b[m
\x1b[49m                  \x1b[49;38;5;15m▀▀▀▀▀▀▀▀▀▀▀▀▀▀\x1b[49m                \x1b[m
\x1b[49m                                                \x1b[m
\x1b[49m                                                \x1b[m
";

const WIDTH: u16 = 50;
const HEIGHT: u16 = 50;

#[derive(Debug)]
pub struct Logo {
  pub init_time: Instant,
  pub is_rendered: bool,
}

impl Default for Logo {
  fn default() -> Self {
    Self {
      init_time: Instant::now(),
      is_rendered: false,
    }
  }
}

impl WidgetRef for Logo {
  fn render_ref(&self, area: Rect, buf: &mut Buffer) {
    let text: Text = LOGO.into_text().expect("failed to parse ANSI");
    text.render(area, buf);
    let message = "Loading...";
    buf.set_string(
      WIDTH / 2 - message.len() as u16 / 2 + area.x,
      HEIGHT,
      message,
      Style::default().fg(Color::Rgb(248, 190, 117)).italic(),
    )
  }
}

impl Logo {
  /// Returns the size of the logo.
  pub fn get_size(&self) -> (u16, u16) {
    (WIDTH, HEIGHT)
  }
}