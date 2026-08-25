use std::io::{self, Stdout};

use crate::app::App;
use crate::core::app::AppCore;
use crate::core::sender::AppSender;
use crate::core::signal::AppSignal;
use crate::keyboard::KeyboardAction;
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
use tokio::sync::mpsc;

type StdoutTerm = Terminal<CrosstermBackend<Stdout>>;
struct Boostrap {
    terminal: StdoutTerm,
    app: App,
    app_sender: AppSender,
}

impl Boostrap {
    fn enter(app: App, app_sender: AppSender) -> Result<Self, anyhow::Error> {
        enable_raw_mode()?;

        let stdout = io::stdout();
        execute!(&stdout, EnterAlternateScreen)?;

        let backend = CrosstermBackend::new(stdout);
        Ok(Self {
            terminal: Terminal::new(backend)?,
            app,
            app_sender,
        })
    }

    fn leave(mut self) -> Result<(), anyhow::Error> {
        disable_raw_mode()?;
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen)?;

        Ok(())
    }

    fn read_key(&mut self) -> io::Result<KeyboardAction> {
        type K = KeyEventKind;
        type E = Event;
        use event::read;

        if let E::Key(key) = read()?
            && key.kind == K::Press
        {
            return Ok(KeyboardAction::keyboard(key.code, &mut self.app));
        }

        Ok(KeyboardAction::None)
    }

    async fn start_main_loop(&mut self) -> Result<(), anyhow::Error> {
        loop {
            self.terminal.draw(|frame| {
                let _ = self.app.render(frame);
            })?;

            match self.read_key()? {
                KeyboardAction::None => {}
                KeyboardAction::Exit => {
                    self.app_sender.send(AppSignal::Exit).await?;
                    break;
                }

                KeyboardAction::Download => {
                    self.app_sender
                        .send(AppSignal::Download {
                            url: self.app.url_input.value.clone(),
                        })
                        .await?;
                }
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
pub async fn boostrap_tui() -> Result<(), anyhow::Error> {
    use tokio::spawn;

    let (sender, receiver) = mpsc::channel(16);
    let app_sender = AppSender::new(sender);
    let app_core = AppCore::new(receiver);
    spawn(app_core.run());

    let mut boostrap = Boostrap::enter(App::default(), app_sender)?;
    boostrap.start_main_loop().await?;
    boostrap.leave()?;

    Ok(())
}
