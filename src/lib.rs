extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;

use gtk::{ApplicationWindow, Button, Orientation};

mod usb;

pub struct Application {
    app: gtk::Application
}

impl Application {
    pub fn new() -> Application {
        let application = gtk::Application::new(
            Some("ru.tvhelp.flasher.intercom"),
            Default::default(),
        ).expect("failed to initialize GTK application");

        application.connect_activate(|app| {
            let window = Application::window(app);
            window.show_all();
        });

        Application { app: application }
    }

    pub fn run(&self) {
        self.app.run(&[]);
    }

    fn window(app: &gtk::Application) -> ApplicationWindow {
        let window = ApplicationWindow::new(app);
        window.set_title("Intercom Flasher");
        window.set_default_size(350, 70);
        window.set_border_width(10);

        let container = gtk::Box::new(Orientation::Vertical, 5);

        let disks_label = gtk::Label::new(Some("Диски"));
        disks_label.set_halign(gtk::Align::Start);

        let disks = gtk::ComboBox::new();

        container.pack_start(&disks_label, false, false, 5);
        container.pack_start(&disks, false, false, 5);


        let button = Button::with_label("Click me!");
        button.set_halign(gtk::Align::End);
        button.connect_clicked(|_| {
            println!("Clicked!");
        });

        container.pack_start(&button, false, false, 5);

        window.add(&container);

        window
    }
}
