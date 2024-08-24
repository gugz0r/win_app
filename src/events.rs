use native_windows_gui as nwg;
use crate::ui::AppUI;
use crate::about;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn setup_events(ui: &AppUI) {
    let open_menu = ui.file_open.handle.clone();
    let exit_menu = ui.file_exit.handle.clone();
    let about_menu = ui.help_about.handle.clone();

    nwg::bind_event_handler(&ui.window.handle, &ui.window.handle, move |evt, _evt_data, handle| {
        match evt {
            nwg::Event::OnMenuItemSelected => {
                if handle == open_menu {
                    println!("Open menu clicked"); // Debugging print
                    open_file_dialog();
                } else if handle == exit_menu {
                    nwg::stop_thread_dispatch();
                } else if handle == about_menu {
                    about::show_about();
                }
            }
            _ => {}
        }
    });
}

fn open_file_dialog() {
    let mut dialog = nwg::FileDialog::default();

    nwg::FileDialog::builder()
        .title("Open File")
        .action(nwg::FileDialogAction::Open)
        .filters("Text Files (*.txt)\0*.txt\0All Files (*.*)\0*.*\0")
        .build(&mut dialog)
        .expect("Failed to build file dialog");

    println!("File dialog opened"); // Debugging print

    if dialog.run(None::<nwg::ControlHandle>) {
        println!("File selected"); // Debugging print
        if let Ok(file_path) = dialog.get_selected_item() {
            let file_path = file_path.into_string().unwrap(); // Convert OsString to String
            open_file_in_new_window(&file_path);
        }
    } else {
        println!("File dialog was canceled or failed to run."); // Debugging print
    }
}

fn open_file_in_new_window(file_path: &str) {
    println!("Opening file: {}", file_path); // Debugging print

    let path = Path::new(&file_path);
    let file_name = path.file_name().unwrap().to_string_lossy().into_owned();

    let mut file_content = String::new();
    if let Ok(mut file) = File::open(&file_path) {
        file.read_to_string(&mut file_content).unwrap();
    }

    let mut new_window = nwg::Window::default();
    nwg::Window::builder()
        .size((600, 400))
        .position((100, 100))  // Position at (100, 100) to ensure it's on-screen
        .title(&file_name)
        .build(&mut new_window)
        .expect("Failed to create new window");

    let mut text_box = nwg::TextBox::default();
    nwg::TextBox::builder()
        .parent(&new_window)
        .size((580, 360))
        .position((10, 10))
        .text(&file_content)
        .readonly(true)
        .flags(nwg::TextBoxFlags::VISIBLE)
        .build(&mut text_box)
        .expect("Failed to create text box");

    new_window.set_visible(true);
    new_window.focus(); // Bring the window to the foreground

    // Start the event loop for the new window
    nwg::dispatch_thread_events();
}
