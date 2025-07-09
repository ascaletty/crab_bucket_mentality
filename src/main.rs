use winit::dpi::PhysicalPosition;
use winit::error::EventLoopError;
use winit::window::{Fullscreen, WindowAttributes};
use xilem::view::{button, flex, label};
use xilem::{EventLoop, WidgetView, Xilem};
#[path = "program_paths.rs"]
mod path;
#[derive(Default)]
struct Counter {
    num: i32,
}

fn app_logic(data: &mut Counter) -> impl WidgetView<Counter> + use<> {
    flex((
        label(format!("{}", data.num)),
        button("increment", |data: &mut Counter| data.num += 1),
    ))
}

fn main() -> Result<(), EventLoopError> {
    let app = Xilem::new(Counter::default(), app_logic);
    let mut window_attributes: WindowAttributes = WindowAttributes::default()
        .with_position(PhysicalPosition { x: 0, y: 0 })
        .with_inner_size(winit::dpi::LogicalSize::new(12.0, 12.0));
    app.run_windowed_in(EventLoop::with_user_event(), window_attributes)?;
    Ok(())
}

