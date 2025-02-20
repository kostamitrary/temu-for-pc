use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Label};
use std::process::Command;
use anyhow::Result;

fn main() -> Result<()> {
    let app = Application::new(Some("com.example.temu_for_pc"), Default::default());
    app.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Temu For PC");
        window.set_default_size(400, 200);

        let label = Label::new(Some("Welcome to Temu For PC"));
        let button = Button::with_label("Run Application");
        button.connect_clicked(|_| {
            let output = Command::new("path/to/application").output().expect("Failed to execute application");
            println!("Output: {:?}", output);
        });

        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
        vbox.pack_start(&label, true, true, 0);
        vbox.pack_start(&button, true, true, 0);
        window.add(&vbox);
        window.show_all();
    });

    app.run();
    Ok(())
}