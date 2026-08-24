use std::io::{self, Stdout};

use crate::app::App;
use crossterm::event::KeyEventKind;
use crossterm::{
    event::{self, Event},
    execute,
    terminal::{
        EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode,
    },
};
use ratatui::{Terminal, backend::CrosstermBackend};

type StdoutTerm = Terminal<CrosstermBackend<Stdout>>;
pub struct Boostrap {
    terminal: StdoutTerm,
    app: App,
}

impl Boostrap {
    fn enter(app: App) -> Result<Self, anyhow::Error> {
        enable_raw_mode()?;

        let stdout = io::stdout();
        execute!(&stdout, EnterAlternateScreen)?;

        let backend = CrosstermBackend::new(stdout);
        Ok(Self {
            terminal: Terminal::new(backend)?,
            app,
        })
    }

    fn leave(mut self) -> Result<(), anyhow::Error> {
        disable_raw_mode()?;
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen)?;

        Ok(())
    }

    fn read_key(&mut self) -> io::Result<bool> {
        type K = KeyEventKind;
        type E = Event;
        use event::read;

        if let E::Key(key) = read()?
            && key.kind == K::Press
            && self.app.url_input.input_handle(key.code)
        {
            return Ok(true);
        }

        Ok(false)
    }

    fn start_main_loop(&mut self) -> Result<(), anyhow::Error> {
        loop {
            self.terminal.draw(|frame| {
                let _ = self.app.render(frame);
            })?;

            if self.read_key()? {
                break;
            }
        }

        Ok(())
    }
}

impl Drop for Boostrap {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(self.terminal.backend_mut(), LeaveAlternateScreen);
    }
}

/// # Errors
/// Boostrap TUI failed.
pub fn boostrap_tui() -> Result<(), anyhow::Error> {
    let mut boostrap = Boostrap::enter(App::default())?;
    boostrap.start_main_loop()?;
    boostrap.leave()?;

    Ok(())
}
