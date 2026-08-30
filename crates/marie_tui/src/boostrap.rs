use std::io::{self, Stdout, stdout};
use std::panic::{set_hook, take_hook};
use std::time::Duration;

use crate::app::App;
use crate::core::app::AppCore;
use crate::core::event::AppEvent;
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
    event_rx: mpsc::Receiver<AppEvent>,
}

impl Boostrap {
    fn enter(
        app: App,
        app_sender: AppSender,
        event_rx: mpsc::Receiver<AppEvent>,
    ) -> Result<Self, anyhow::Error> {
        enable_raw_mode()?;

        let stdout = io::stdout();
        execute!(&stdout, EnterAlternateScreen)?;

        let backend = CrosstermBackend::new(stdout);
        Ok(Self {
            terminal: Terminal::new(backend)?,
            app,
            app_sender,
            event_rx,
        })
    }

    fn panic_hook() {
        let original_hook = take_hook();
        set_hook(Box::new(move |info| {
            let _ = disable_raw_mode();
            let _ = execute!(stdout(), LeaveAlternateScreen);
            original_hook(info);
        }));
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
            if let Ok(event) = self.event_rx.try_recv() {
                match event {
                    AppEvent::Log(text) => self.app.push_log(text),
                    AppEvent::Error(e) => self.app.push_log(e),
                }
            }

            self.terminal.draw(|frame| {
                self.app.render(frame);
            })?;

            if event::poll(Duration::from_millis(50))? {
                match self.read_key()? {
                    KeyboardAction::None => {}
                    KeyboardAction::Exit => {
                        self.app_sender.send(AppSignal::Exit).await?;
                        break;
                    }

                    KeyboardAction::Download => {
                        let url = self.app.url_value.clone();
                        if url.is_empty() {
                            self.app.push_log("URL is empty.");
                            continue;
                        }

                        self.app.push_log(format!("Fetching {url}"));
                        self.app_sender
                            .send(AppSignal::Download { url })
                            .await?;
                    }
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

    let (cmd_tx, cmd_rx) = mpsc::channel(16);
    let (evt_tx, evt_rx) = mpsc::channel(16);
    let app_sender = AppSender::new(cmd_tx);
    let app_core = AppCore::new(cmd_rx, evt_tx);

    spawn(app_core.run());

    let mut boostrap = Boostrap::enter(App::default(), app_sender, evt_rx)?;
    Boostrap::panic_hook();
    boostrap.start_main_loop().await?;
    boostrap.leave()?;

    Ok(())
}
