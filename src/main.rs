use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};

fn main() {
    // Create a new application
    let app = Application::new(Some("online.sashin.japanesewordtransformer"), Default::default());
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(application: &Application) {
    // Create a window
    let window = ApplicationWindow::new(application);

    // Set the window title
    window.set_title(Some("japanese Word Transformer"));

    let button = Button::with_label("Translate \"Hello World!\"");

    // Set the button margins
    button.set_margin_top(12);
    button.set_margin_bottom(12);
    button.set_margin_start(12);
    button.set_margin_end(12);

    // Connect callback
    button.connect_clicked(move |button| {
        // Set the label to "Hello World!" after the button has been clicked on
        button.set_label("こんにちは世界!");
    });

    // Add button
    window.set_child(Some(&button));
    window.present();
}