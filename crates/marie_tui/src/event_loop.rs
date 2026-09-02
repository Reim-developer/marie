use crossterm::event::{Event, EventStream, KeyCode, KeyEventKind};
use futures_util::{FutureExt, StreamExt};
use tokio::sync::mpsc;

use crate::{
    app::App,
    core::{event::AppEvent, sender::AppSender, signal::AppSignal},
    keyboard::KeyboardAction,
    log_entry::LogEntry,
    tui::TuiGuard,
};

enum Input {
    Keyboard(KeyCode),
    Core(AppEvent),
}

pub struct EventLoop {
    app: App,
    app_sender: AppSender,
    event_rx: mpsc::Receiver<AppEvent>,
}

impl EventLoop {
    #[must_use]
    pub const fn new(
        app: App,
        app_sender: AppSender,
        event_rx: mpsc::Receiver<AppEvent>,
    ) -> Self {
        Self {
            app,
            app_sender,
            event_rx,
        }
    }

    /// # Errors
    /// Terminal draw or channel send failed.
    pub async fn run(
        mut self,
        tui: &mut TuiGuard,
    ) -> Result<(), anyhow::Error> {
        let mut reader = EventStream::new();

        loop {
            tui.terminal().draw(|frame| {
                self.app.render(frame);
            })?;

            let input = tokio::select! {
                maybe_event = reader.next().fuse() => {
                    match maybe_event {
                        Some(Ok(Event::Key(key))) if key.kind == KeyEventKind::Press => {
                            Input::Keyboard(key.code)
                        }
                        _=> continue,
                    }
                }

                maybe_event = self.event_rx.recv() => {
                    match maybe_event {
                        Some(event) => Input::Core(event),
                        None => break,
                    }
                }
            };

            if self.handle_input(input).await? {
                break;
            }
        }

        Ok(())
    }

    async fn handle_input(
        &mut self,
        input: Input,
    ) -> Result<bool, anyhow::Error> {
        match input {
            Input::Keyboard(key) => {
                match KeyboardAction::keyboard(key, &mut self.app) {
                    KeyboardAction::None => {}
                    KeyboardAction::Exit => {
                        self.app_sender.send(AppSignal::Exit).await?;

                        return Ok(true);
                    }

                    KeyboardAction::Download => {
                        let ctx = self.app.ctx();
                        let url = ctx.url_value();
                        let scope = ctx.features_selected();

                        self.app.push_log(LogEntry::Info(format!(
                            "Fetching {url}"
                        )));

                        ctx.set_busy(true);
                        self.app_sender
                            .send(AppSignal::Download { url, scope })
                            .await?;
                    }
                }
            }

            Input::Core(event) => {
                let ctx = self.app.ctx();
                ctx.set_busy(false);

                match event {
                    AppEvent::Log(text) => {
                        self.app.push_log(LogEntry::Success(text));
                    }
                    AppEvent::Error(e) => self.app.push_log(LogEntry::Error(e)),
                }
            }
        }

        Ok(false)
    }
}
