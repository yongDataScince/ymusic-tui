use tui::{Frame, backend::Backend, layout::{Layout, Direction, Constraint, Alignment}, widgets::{Paragraph, Block, Borders}, style::{Style, Color}};

use super::styles::Styles;

pub fn draw<B>(rect: &mut Frame<B>, app: App<B>, title: String, styles: Styles) where B: Backend {
  let size = rect.size();
  let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3)].as_ref())
        .split(size);

    // Title block
    let title = draw_title(title, styles);
    rect.render_widget(title, chunks[0]);
}

fn draw_title<'a>(title: String, styles: Styles) -> Paragraph<'a> {
  Paragraph::new(title.as_str())
      .style(Style::default().fg(styles.text_color))
      .alignment(Alignment::Center)
      .block(
          Block::default()
              .borders(Borders::ALL)
              .style(Style::default().fg(Color::White))
              .border_type(styles.border_type),
      )
}