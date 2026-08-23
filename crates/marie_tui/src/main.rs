use marie_tui::boostrap::boostrap_tui;

fn main() {
    boostrap_tui().map_err(|e| panic!("Failed to boostrap Marie Tui {e}"));
}
