use gtk::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};
use glib::{Properties, ParamSpec, Value};
use std::cell::RefCell;
use crate::ui;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate, Properties)]
    #[properties(wrapper_type = super::KtWindow)]
    #[template(resource = "/gay/leahc/Kitchen/window.ui")]
    pub struct KtWindow {
        #[property(name = "main-text", get, set)]
        pub main_text: RefCell<String>,
        // Template widgets
        #[template_child]
        pub project: TemplateChild<ui::project::KtProjectItem>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for KtWindow {
        const NAME: &'static str = "KtWindow";
        type Type = super::KtWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for KtWindow {
        fn properties() -> &'static [ParamSpec] {
            Self::derived_properties()
        }
        fn set_property(&self, id: usize, value: &Value, pspec: &ParamSpec) {
            self.derived_set_property(id, value, pspec)
        }
        fn property(&self, id: usize, pspec: &ParamSpec) -> Value {
            self.derived_property(id, pspec)
        }
    }

    impl WidgetImpl for KtWindow {}
    impl WindowImpl for KtWindow {}
    impl ApplicationWindowImpl for KtWindow {}
    impl AdwApplicationWindowImpl for KtWindow {}
}

glib::wrapper! {
    pub struct KtWindow(ObjectSubclass<imp::KtWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,        @implements gio::ActionGroup, gio::ActionMap;
}

impl KtWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .property("main-text", "fortnite battle pass")
            .build()
    }
}
