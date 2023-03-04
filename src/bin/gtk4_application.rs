use gtk4::{
    Application,
    prelude::*,
};

use rust_gtk4_boilerplate::app::gtk4_application::window::RustGtk4BoilerplateWindow;

fn main() {
    // Create a new application
    let app = Application::builder().application_id(rust_gtk4_boilerplate::app::gtk4_application::APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    // Create a window and set the title
    let window = RustGtk4BoilerplateWindow::new(app);

    // Present window
    window.present();
}