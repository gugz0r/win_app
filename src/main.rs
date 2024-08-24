mod ui;
mod events;
mod about;

use native_windows_gui as nwg;

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");

    let ui = ui::AppUI::new();
    events::setup_events(&ui);

    nwg::dispatch_thread_events();
}
