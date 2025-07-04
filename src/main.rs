use gtk::{
    Application, ApplicationWindow, Button, ListBox, PolicyType, ScrolledWindow, TextView, glib,
};
use gtk::{Window, prelude::*};

const APP_ID: &str = "org.gtk_rs.HelloWorld2";
use std::collections::HashMap;
use std::os::unix::process::CommandExt;
use std::process::Command;
#[path = "program_paths.rs"]
mod path;
fn main() -> glib::ExitCode {
    path::parse_programs();
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    let mut programs = HashMap::new();
    programs = path::read_cache(programs).expect("failed to parse csv");
    let program_list = ListBox::new();
    for (program, exec) in programs {
        let label = Button::with_label(&program);
        label.connect_clicked(move |label| {
            label.set_label("hellogheran");
            let mut cmd = Command::new(&exec);
            cmd.exec();
        });
        program_list.append(&label);
    }
    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never) // Disable horizontal scrolling
        .min_content_width(360)
        .child(&program_list)
        .build();

    let text_input = TextView::builder()
        .name("drun")
        .receives_default(true)
        // .opacity(50)
        .build();
    let window = Window::builder()
        .application(app)
        .title("My GTK App")
        .child(&text_input)
        // .halign("left")
        .child(&scrolled_window)
        .default_height(300)
        // .opacity(50)
        .default_width(360)
        .resizable(true)
        .build();

    // Present window
    window.present();
}
