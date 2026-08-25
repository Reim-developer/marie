use marie_tui::boostrap::boostrap_tui;

#[tokio::main]
async fn main() {
    boostrap_tui()
        .await
        .unwrap_or_else(|e| panic!("Failed to boostrap Marie Tui {e}"));
}
