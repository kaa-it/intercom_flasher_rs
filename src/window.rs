use gtk::*;
use gtk::prelude::*;

use crate::usb;

pub struct Window {
    window: ApplicationWindow,
    devices_store: ListStore,
}

impl  Window {
    pub fn new(app: &gtk::Application) -> Window {
        Window {
            window: ApplicationWindow::new(app),
            devices_store: ListStore::new(&[
                String::static_type(),
                String::static_type(),
                String::static_type(),
            ]),
        }
    }

    pub fn build_ui(&self) {
        self.window.set_title("Intercom Flasher");
        self.window.set_default_size(350, 70);
        self.window.set_border_width(10);

        if let Err(e) = self.window.set_icon_from_file("icons/logo.png") {
            println!("{}", e.to_string());
        }

        let container = gtk::Box::new(Orientation::Vertical, 5);

        let disks_label = Label::new(Some("Устройства"));
        disks_label.set_halign(Align::Start);

        Window::update_devices(&self.devices_store);

        let titles = &["Путь", "Модель", "Размер (Гб)"];

        let disks = TreeView::new();

        for (i, &title) in titles.iter().enumerate() {
            let column = TreeViewColumn::new();
            let cell = CellRendererText::new();

            column.pack_start(&cell, true);
            column.add_attribute(&cell, "text", i as i32);
            column.set_title(title);
            disks.append_column(&column);
        }

        disks.set_model(Some(&self.devices_store));

        container.pack_start(&disks_label, false, false, 5);
        container.pack_start(&disks, true, true, 5);

        let refresh_button = Button::with_label("Обновить");
        refresh_button.set_halign(Align::End);

        let store = self.devices_store.clone();

        refresh_button.connect_clicked(move |_| {
            Window::update_devices(&store);
        });

        container.pack_start(&refresh_button, false, false, 5);

        self.window.add(&container);
    }

    pub fn show(&self) {
        self.window.show_all();
    }

    fn update_devices(store: &ListStore) {
        store.clear();

        for entry in usb::get_devices().iter() {
            store.insert_with_values(
                None,
                &[0, 1, 2],
                &[&entry[0], &entry[1], &entry[2]],
            );
        }
    }
}