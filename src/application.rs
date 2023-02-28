
use gtk::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};

use crate::config::VERSION;
use crate::KtWindow;

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct KitchenApplication {}

    #[glib::object_subclass]
    impl ObjectSubclass for KitchenApplication {
        const NAME: &'static str = "KitchenApplication";
        type Type = super::KitchenApplication;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for KitchenApplication {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.setup_gactions();
            obj.set_accels_for_action("app.quit", &["<primary>q"]);
        }
    }

    impl ApplicationImpl for KitchenApplication {
        // We connect to the activate callback to create a window when the application
        // has been launched. Additionally, this callback notifies us when the user
        // tries to launch a "second instance" of the application. When they try
        // to do that, we'll just present any existing window.
        fn activate(&self) {
            let application = self.obj();
            // Get the current window or create one if necessary
            let window = if let Some(window) = application.active_window() {
                window
            } else {
                let window = KtWindow::new(&*application);
                window.upcast()
            };

            // Ask the window manager/compositor to present the window
            window.present();
        }
    }

    impl GtkApplicationImpl for KitchenApplication {}
    impl AdwApplicationImpl for KitchenApplication {}
}

glib::wrapper! {
    pub struct KitchenApplication(ObjectSubclass<imp::KitchenApplication>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl KitchenApplication {
    pub fn new(application_id: &str, flags: &gio::ApplicationFlags) -> Self {
        glib::Object::builder()
            .property("application-id", application_id)
            .property("flags", flags)
            .build()
    }

    fn setup_gactions(&self) {
        let quit_action = gio::ActionEntry::builder("quit")
            .activate(move |app: &Self, _, _| app.quit())
            .build();
        let about_action = gio::ActionEntry::builder("about")
            .activate(move |app: &Self, _, _| app.show_about())
            .build();
        self.add_action_entries([quit_action, about_action]);
    }

    fn show_about(&self) {
        let window = self.active_window().unwrap();
        let about = adw::AboutWindow::builder()
            .transient_for(&window)
            .application_name("Kitchen")
            .application_icon("gay.leahc.Kitchen")
            .developer_name("Leah Clark")
            .version(VERSION)
            .developers(vec!["Leah Clark"])
            .designers(vec!["Leah Clark"])
            .copyright("© 2023 leah!")
            .build();

        about.present();
    }
}
