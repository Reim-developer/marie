use tokio::sync::mpsc;

use crate::{
    app::App,
    core::{app::AppCore, sender::AppSender},
    event_loop::EventLoop,
    tui::TuiGuard,
};

/// # Errors
/// Boostrap TUI failed.
pub async fn boostrap_tui() -> Result<(), anyhow::Error> {
    let (cmd_tx, cmd_rx) = mpsc::channel(16);
    let (evt_tx, evt_rx) = mpsc::channel(16);

    let app_sender = AppSender::new(cmd_tx);
    let app_core = AppCore::new(cmd_rx, evt_tx);

    tokio::spawn(app_core.run());
    TuiGuard::setup_panic_hook();

    let mut tui = TuiGuard::new()?;
    let event_loop = EventLoop::new(App::default(), app_sender, evt_rx);

    event_loop.run(&mut tui).await?;

    Ok(())
}
