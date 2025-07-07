
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
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) { 
    let mut programs_vec: Vec<path::ProgramData> = Vec::new();
    programs_vec=path::parse_programs(programs_vec).expect("failed to parse program");
    let program_list = ListBox::new();
    for program_data in programs_vec {
        let label = Button::with_label(program_data.name.as_str());
        if program_data.terminal == true{
         label.connect_clicked(move |label| {
            let mut temp = Command::new("kitty");
            let mut cmd= temp.args([program_data.exec.as_str()]);
            cmd.exec();
         });
        }
        else{
        label.connect_clicked(move |label| {
            let mut cmd = Command::new(program_data.exec.as_str());
            cmd.exec();
        });
        }
        program_list.append(&label);
    }
    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never) // Disable horizontal scrolling
        .min_content_width(360)
        .child(&program_list)
        .build();

    let text_input = TextView::builder()
        .name("drun")
        .top_margin(300)
        .bottom_margin(300)
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

