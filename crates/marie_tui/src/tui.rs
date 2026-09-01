use std::{
    io::{self, Stdout, stdout},
    panic::{set_hook, take_hook},
};

use crossterm::{
    execute,
    terminal::{
        EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode,
    },
};
use ratatui::{Terminal, backend::CrosstermBackend};

pub struct TuiGuard {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl TuiGuard {
    /// # Errors
    /// Setup terminal failed.
    pub fn new() -> io::Result<Self> {
        enable_raw_mode()?;
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen)?;

        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        Ok(Self { terminal })
    }

    pub const fn terminal(
        &mut self,
    ) -> &mut Terminal<CrosstermBackend<Stdout>> {
        &mut self.terminal
    }

    pub fn setup_panic_hook() {
        let original = take_hook();
        set_hook(Box::new(move |info| {
            let _ = disable_raw_mode();
            let _ = execute!(io::stdout(), LeaveAlternateScreen);

            original(info);
        }));
    }
}

impl Drop for TuiGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(self.terminal.backend_mut(), LeaveAlternateScreen);
    }
}
