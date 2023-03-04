use gtk4::{
    // gio,
    glib,
    ApplicationWindow,
    // Label,
    // HeaderBar,
    Inhibit,
    CompositeTemplate,
    template_callbacks,
    prelude::*,
    subclass::prelude::*,
};

use glib::once_cell::sync::OnceCell;
use gio::Settings;

#[derive(Default, CompositeTemplate)]
#[template(file = "window.ui")]
pub struct RustGtk4BoilerplateWindow {
    #[template_child(id = "label")]
    pub main_menu_bar: TemplateChild<gtk4::Label>,

    pub settings: OnceCell<Settings>,
}

#[glib::object_subclass]
impl ObjectSubclass for RustGtk4BoilerplateWindow {
    const NAME: &'static str = "RustGtk4BoilerplateWindow";
    type Type = super::RustGtk4BoilerplateWindow;
    type ParentType = ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        RustGtk4BoilerplateCallbacks::bind_template_callbacks(klass);
    }

    // You must call `Widget`'s `init_template()` within `instance_init()`.
    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

struct RustGtk4BoilerplateCallbacks {}

#[template_callbacks(functions)]
impl RustGtk4BoilerplateCallbacks {
    // #[template_callback]
    // fn to_string(value: &glib::Value) -> String {
    //     if let Ok(value) = value.get::<u64>() {
    //         value.to_string()
    //     } else if let Ok(value) = value.get::<&str>() {
    //         value.to_owned()
    //     } else {
    //         "".into()
    //     }
    // }
    // #[template_callback]
    // fn strlen(s: &str) -> u64 {
    //     s.len() as u64
    // }
    // #[template_callback(name = "concat_strs")]
    // fn concat(#[rest] values: &[glib::Value]) -> String {
    //     let mut res = String::new();
    //     for (index, value) in values.iter().enumerate() {
    //         res.push_str(value.get::<&str>().unwrap_or_else(|e| {
    //             panic!("Expected string value for argument {}: {}", index, e);
    //         }));
    //     }
    //     res
    // }
    // #[template_callback(function = false, name = "reset_entry")]
    // fn reset(entry: &gtk4::Entry) {
    //     entry.set_text("Nothing");
    // }
}

impl ObjectImpl for RustGtk4BoilerplateWindow {

    fn constructed(&self, obj: &Self::Type) {
        // obj.init_label();
        self.parent_constructed(obj);

        // Load latest window state
        obj.setup_settings();
        obj.load_window_size();

        // Add actions
        // obj.setup_actions();
    }

}

// impl BoxImpl for RustGtk4BoilerplateWindow {}

impl WidgetImpl for RustGtk4BoilerplateWindow {}

impl WindowImpl for RustGtk4BoilerplateWindow {
    // Save window state right before the window will be closed
    fn close_request(&self, obj: &Self::Type) -> Inhibit {
        // Save window size
        obj
            .save_window_size()
            .expect("Failed to save window state");

        // Don't inhibit the default handler
        Inhibit(false)
    }
}

impl ApplicationWindowImpl for RustGtk4BoilerplateWindow {}