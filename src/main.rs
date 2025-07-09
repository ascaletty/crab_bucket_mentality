use gtk::CssProvider;
use gtk::gdk::Display;
use gtk::prelude::*;
use gtk::{Application, Button, ListBox, PolicyType, Text, TextBuffer, Window, glib};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
const APP_ID: &str = "org.gtk_rs.HelloWorld2";
use std::path::Path;
use std::process::Command;

use crate::path::ProgramData;
#[path = "program_paths.rs"]
mod path;
fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}
fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    let path = Path::new("src/style.css");
    provider.load_from_path(path);

    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(app: &Application) {
    let text_input = Text::builder()
        .hexpand(true)
        .vexpand(true)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Start)
        .activates_default(true)
        .receives_default(true)
        .focusable(true)
        .build();
    let cache_dir = Path::new("/home/ascaletty23/.cache/crab_bucket_mentality.cache");
    let temp_dir = Path::new("/home/ascaletty23/.cache/crab_bucket_mentality1.cache");
    let mut programs_vec: Vec<path::ProgramData> = Vec::new();
    programs_vec = path::parse_programs(programs_vec).expect("failed to parse program");

    if gtk::Text::activate(&text_input) {
        let mut cmd = Command::new("kitty");
        cmd.spawn();
        print!("command hit");
    }
    // let program_list = ListBox::new();
    // program_list.add_css_class("program_list");
    //
    // program_list.append(&text_input);
    // for program_data in programs_vec {
    //     let label = Button::with_label(program_data.name.as_str());
    //     if !program_data.no_display {
    //         if program_data.terminal {
    //             label.connect_clicked(move |_label| {
    //                 let mut temp = Command::new("kitty");
    //                 let cmd = temp.args([program_data.exec.as_str()]);
    //                 path::edit_cache(&program_data.name, cache_dir, temp_dir)
    //                     .expect("failed edit cache");
    //                 cmd.spawn().expect("failed execution");
    //             });
    //         } else {
    //             label.connect_clicked(move |_label| {
    //                 let mut cmd = Command::new(program_data.exec.as_str());
    //                 path::edit_cache(&program_data.name, cache_dir, temp_dir)
    //                     .expect("failed edit cache");
    //                 cmd.spawn().expect("failed exec");
    //             });
    //         }
    //         program_list.append(&label);
    //
    //         program_list.select_row(program_list.row_at_index(1).as_ref());
    //     }
    // }
    // let scrolled_window = ScrolledWindow::builder()
    //     .hscrollbar_policy(PolicyType::Never) // Disable horizontal scrolling
    //     .min_content_width(360)
    //     .child(&program_list)
    //     .build();

    let window = Window::builder()
        .application(app)
        .title("My GTK App")
        .default_height(20)
        // .halign("left")
        // .child(&scrolled_window)
        .child(&text_input)
        .halign(gtk::Align::Fill)
        .valign(gtk::Align::Fill)
        .decorated(false)
        // .opacity(50)
        .fullscreened(false)
        .resizable(true)
        .build();
    window.init_layer_shell();
    window.set_layer(Layer::Overlay);
    window.auto_exclusive_zone_enable();
    window.set_margin(Edge::Left, 10);
    window.set_margin(Edge::Right, 10);
    window.set_margin(Edge::Top, 10);

    // ... or like this
    // Anchors are if the window is pinned to each edge of the output
    let anchors = [
        (Edge::Left, true),
        (Edge::Right, true),
        (Edge::Top, true),
        (Edge::Bottom, false),
    ];

    for (anchor, state) in anchors {
        window.set_anchor(anchor, state);
    }
    window.set_keyboard_mode(gtk4_layer_shell::KeyboardMode::OnDemand);

    // Present window

    window.show();
}
