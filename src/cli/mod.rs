mod items;
mod styles;
mod prelude;

use std::{io::stdout, cell::RefCell, rc::Rc};
use crossterm::terminal::enable_raw_mode;
use tui::{backend::CrosstermBackend, Terminal};
use prelude::*;

use self::styles::Styles;

pub struct Cli {
    pub styles: Styles,
}

impl Cli {
  pub fn start(title: String, app: Rc<RefCell<App>>) -> Result<(), Box<dyn std::error::Error>> {
    let stdout = stdout();
    enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;
    terminal.hide_cursor()?;

    loop {
        let app = app.borrow();
        terminal.draw(|rect| draw(rect, &app))?;
    }

    // Restore the terminal and close application
    terminal.clear()?;
    terminal.show_cursor()?;
  crossterm::terminal::disable_raw_mode()?;

    Ok(())
  }
}