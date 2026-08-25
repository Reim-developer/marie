use tokio::sync::mpsc;

use crate::{core::signal::AppSignal, utils::not_used};

type SignalReceiver = mpsc::Receiver<AppSignal>;
pub struct AppCore {
    receiver: SignalReceiver,
}

impl AppCore {
    #[must_use]
    pub const fn new(receiver: SignalReceiver) -> Self {
        Self { receiver }
    }

    pub async fn run(mut self) {
        type A = AppSignal;

        while let Some(signal) = self.receiver.recv().await {
            match signal {
                A::Download { url } => {
                    not_used(&url);
                    /* Download was not implemented in this time */
                }

                AppSignal::Exit => break,
            }
        }
    }
}
