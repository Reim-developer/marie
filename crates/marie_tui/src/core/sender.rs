use tokio::sync::mpsc;
use tokio::sync::mpsc::error::SendError;

use crate::core::signal::AppSignal;

type SignalSender = mpsc::Sender<AppSignal>;
type SendResult = Result<(), SendError<AppSignal>>;
pub struct AppSender {
    sender: SignalSender,
}

impl AppSender {
    #[must_use]
    pub const fn new(sender: SignalSender) -> Self {
        Self { sender }
    }

    /// # Errors
    /// Send signal failed.
    pub async fn send(&self, signal: AppSignal) -> SendResult {
        self.sender.send(signal).await
    }
}
