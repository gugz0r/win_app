use native_windows_gui as nwg;
use nwg::{Window, Menu, MenuItem};

pub struct AppUI {
    pub window: Window,
    pub file_menu: Menu,
    pub file_open: MenuItem,
    pub file_exit: MenuItem,
    pub help_menu: Menu,
    pub help_about: MenuItem,
}

impl AppUI {
    pub fn new() -> Self {
        let mut window = Window::default();
        let mut file_menu = Menu::default();
        let mut file_open = MenuItem::default();
        let mut file_exit = MenuItem::default();
        let mut help_menu = Menu::default();
        let mut help_about = MenuItem::default();

        nwg::Window::builder()
            .size((600, 400))
            .position((300, 300))
            .title("My NWG App")
            .build(&mut window)
            .expect("Failed to create window");

        nwg::Menu::builder()
            .text("File")
            .parent(&window)
            .build(&mut file_menu)
            .expect("Failed to create file menu");

        nwg::MenuItem::builder()
            .text("Open")
            .parent(&file_menu)
            .build(&mut file_open)
            .expect("Failed to create open menu item");

        nwg::MenuItem::builder()
            .text("Exit")
            .parent(&file_menu)
            .build(&mut file_exit)
            .expect("Failed to create exit menu item");

        nwg::Menu::builder()
            .text("Help")
            .parent(&window)
            .build(&mut help_menu)
            .expect("Failed to create help menu");

        nwg::MenuItem::builder()
            .text("About")
            .parent(&help_menu)
            .build(&mut help_about)
            .expect("Failed to create about menu item");

        Self {
            window,
            file_menu,
            file_open,
            file_exit,
            help_menu,
            help_about,
        }
    }
}
