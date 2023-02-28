use adw::subclass::prelude::*;
use gtk::{gio, glib};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/gay/leahc/Kitchen/ui/project.ui")]
    pub struct KtProjectItem {}

    #[glib::object_subclass]
    impl ObjectSubclass for KtProjectItem {
        const NAME: &'static str = "KtProjectItem";
        type Type = super::KtProjectItem;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for KtProjectItem {}
    impl WidgetImpl for KtProjectItem {}
    impl BoxImpl for KtProjectItem {}
}

glib::wrapper! {
    pub struct KtProjectItem(ObjectSubclass<imp::KtProjectItem>)
        @extends gtk::Widget, gtk::Box,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl KtProjectItem {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
