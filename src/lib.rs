mod state;
mod input;
mod rendering;

use winit::{
    event::{Event, WindowEvent, DeviceEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use state::State;

pub fn run() {
    pollster::block_on(async_run());
}

async fn async_run() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let mut state = State::new(&window).await;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent { event, .. } => {
                state.input.process_window_events(&event);
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        state.resize(physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        state.resize(*new_inner_size);
                    }
                    _ => {}
                }
            }
            Event::DeviceEvent { event, .. } => {
                state.input.process_device_events(&event);
            }
            Event::MainEventsCleared => {
                if state.input.quit {
                    *control_flow = ControlFlow::Exit;
                } else {
                    state.update(&window);
                    if let Err(e) = state.render() {
                        eprintln!("Render error: {:?}", e);
                    }
                    window.request_redraw();
                }
            }
            _ => {}
        }
    });
}