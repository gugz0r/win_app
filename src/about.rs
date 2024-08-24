use native_windows_gui as nwg;

pub fn show_about() {
    nwg::simple_message("About", "test app\nVersion 0.1.0\n(c) 2024 thegug");
}
