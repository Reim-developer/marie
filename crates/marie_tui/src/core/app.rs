use marie_core::net::HttpClient;
use tokio::sync::mpsc;

use crate::core::{event::AppEvent, signal::AppSignal};

type EventSender = mpsc::Sender<AppEvent>;
type SignalReceiver = mpsc::Receiver<AppSignal>;
pub struct AppCore {
    receiver: SignalReceiver,
    event_tx: EventSender,
    client: HttpClient,
}

impl AppCore {
    #[must_use]
    pub fn new(receiver: SignalReceiver, event_tx: EventSender) -> Self {
        Self {
            receiver,
            event_tx,
            client: HttpClient::new(),
        }
    }

    async fn download(&self, url: String) {
        match self.client.fetch_text(url).await {
            Ok(text) => {
                let _ = self.event_tx.send(AppEvent::Log(text)).await;
            }

            Err(e) => {
                let _ =
                    self.event_tx.send(AppEvent::Error(format!("{e}"))).await;
            }
        }
    }

    pub async fn run(mut self) {
        type A = AppSignal;

        while let Some(signal) = self.receiver.recv().await {
            match signal {
                A::Download { url } => {
                    self.download(url).await;
                }

                AppSignal::Exit => break,
            }
        }
    }
}
