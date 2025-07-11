use hyprland::event_listener::AsyncEventListener;
pub async fn check_workspace_update() -> hyprland::Result<()> {
    let mut event_listener = AsyncEventListener::new();

    event_listener.add_active_window_changed_handler(|data| println!("{data:#?}"));
    event_listener.add_fullscreen_state_changed_handler(|fstate| {
        println!("Window {} fullscreen", if fstate { "is" } else { "is not" })
    });
    // event_listener.add_active_monitor_change_handler(|state| println!("Monitor state: {state:#?}"));
    event_listener.add_workspace_changed_handler(|id| println!("workspace changed to {id:?}"));
    event_listener.add_active_window_changed_handler(|state| print!("hi"));
    print!("hi program running hehe");

    // and execute the function
    // here we are using the blocking variant
    // but there is a async version too
    event_listener.start_listener_async().await?;

    Ok(())
}
