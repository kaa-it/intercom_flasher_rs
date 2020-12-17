extern crate gtk;
extern crate gio;

mod usb;
mod window;

use window::Window;

use gio::ApplicationExt;
use gio::prelude::ApplicationExtManual;

pub struct Application {
    app: gtk::Application,
}

impl Application {
    pub fn new() -> Application {
        let application = gtk::Application::new(
            Some("ru.tvhelp.flasher.intercom"),
            Default::default(),
        ).expect("failed to initialize GTK application");

        let app = Application { app: application };

        app.app.connect_activate(|a| {
            let window = Application::window(&a);
            window.build_ui();
            window.show();
        });

        app
    }

    pub fn run(&self) {
        self.app.run(&[]);
    }

    fn window(app: &gtk::Application) -> Window {
        Window::new(app)
    }
}
