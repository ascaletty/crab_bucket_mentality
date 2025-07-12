use hyprland::data::*;
use hyprland::event_listener::EventListener;
use hyprland::prelude::*;
use relm4::Worker;
use relm4::prelude::*;
#[derive(Debug)]
pub enum WorkspaceMsg {
    CurrentWorkspaceUpdate,
}
#[derive(Debug)]
pub enum HyprMsg {
    CurrentWorkspaceUpdate,
}
pub struct Hypr {
    pub currentWorkspace: i32,
    pub activeWorkspaces: Vec<hyprland::data::Workspace>,
}
impl Worker for Hypr {
    type Init = Hypr;
    type Input = HyprMsg;
    type Output = WorkspaceMsg;
    fn init(_init: Self::Init, sender: ComponentSender<Self>) -> Self {
        let mut event_listener = EventListener::new();
        event_listener.add_workspace_changed_handler(move |id| {
            sender.output(WorkspaceMsg::CurrentWorkspaceUpdate);
        });

        event_listener.start_listener();
        Hypr {
            currentWorkspace: Client::get_active().unwrap().unwrap().workspace.id,
            activeWorkspaces: Workspaces::get().unwrap().to_vec(),
        }
    }
    fn update(&mut self, msg: HyprMsg, sender: ComponentSender<Self>) {
        match msg {
            HyprMsg::CurrentWorkspaceUpdate => sender.output(WorkspaceMsg::CurrentWorkspaceUpdate),
        };
    }
}
