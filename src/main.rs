use gtk::prelude::*;

use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};
use hyprland::data::*;
use hyprland::prelude::*;
use relm4::Worker;
use relm4::WorkerController;
use relm4::{prelude::*, set_global_css_from_file};
use std::convert::identity;
use std::process::Command;

use crate::path::Hypr;
#[path = "workspaces.rs"]
mod path;
#[derive(Debug)]
enum AppMsg {
    Enter,
    Current_string,
}
struct AppData {
    current_string: gtk::EntryBuffer,
    terminal: bool,
    worker: WorkerController<path::Hypr>,
}
struct AppWidgets {
    text_input: gtk::Text,
}

impl SimpleComponent for AppData {
    type Input = AppMsg;
    type Output = String;
    type Init = AppData;
    type Root = gtk::Window;
    type Widgets = AppWidgets;
    fn init_root() -> Self::Root {
        let window = gtk::Window::builder()
            .default_height(15)
            .hexpand(true)
            .build();
        window.init_layer_shell();

        // Display above normal windows
        window.set_layer(Layer::Background);

        // Push other windows out of the way
        window.auto_exclusive_zone_enable();

        // The margins are the gaps around the window's edges
        // Margins and anchors can be set like this...
        window.set_margin(Edge::Left, 0);
        window.set_margin(Edge::Right, 0);
        window.set_margin(Edge::Top, 0);
        window.set_margin(Edge::Bottom, 0);
        window.set_keyboard_mode(KeyboardMode::OnDemand);
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
        window
    }
    fn init(
        launcher: Self::Init,
        window: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        set_global_css_from_file("src/style.css").expect("failed css setting");
        let mut model = AppData {
            current_string: launcher.current_string,
            terminal: launcher.terminal,
            worker: Hypr::builder()
                .detach_worker(Hypr {
                    currentWorkspace: Client::get_active().unwrap().unwrap().workspace.id,
                    activeWorkspaces: Workspaces::get().unwrap().to_vec(),
                })
                .forward(sender.input_sender(), identity),
        };
        let vbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .height_request(15)
            .build();
        vbox.add_css_class("main_box");
        let left_box = gtk::Label::builder()
            .halign(gtk::Align::Start)
            .label("11:11")
            .width_request(75)
            .height_request(15)
            .hexpand(true)
            .build();
        let middle_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .halign(gtk::Align::Center)
            .baseline_position(gtk::BaselinePosition::Center)
            .height_request(15)
            .hexpand(true)
            .width_request(50)
            .build();
        let right_box = gtk::Label::builder()
            .halign(gtk::Align::End)
            .height_request(15)
            .hexpand(true)
            .width_request(75)
            .build();

        let text_input = gtk::Text::builder()
            .buffer(&model.current_string)
            .halign(gtk::Align::Center)
            .hexpand(true)
            .build();
        window.set_child(Some(&vbox));
        vbox.append(&left_box);
        vbox.append(&middle_box);
        vbox.append(&right_box);
        middle_box.append(&text_input);
        text_input.connect_activate(move |text_input| {
            print!("enter hit");
            sender.input(AppMsg::Enter);
        });
        let widgets = AppWidgets { text_input };
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            AppMsg::Enter => {
                let mut cmd = Command::new(self.current_string.text().to_string());
                self.current_string.set_text("");
                cmd.spawn();
            }
            AppMsg::Current_string => {}
        }
    }
}

fn main() {
    let app = RelmApp::new("relm4.bar.app");
    app.run::<AppData>(AppData {
        current_string: gtk::EntryBuffer::builder().build(),
        terminal: false,
        worker: path::Hypr::init(),
    });
}
