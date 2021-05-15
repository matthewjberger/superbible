use anyhow::Result;
use gl::types::*;
use glutin::{
    dpi::PhysicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    ContextBuilder,
};

fn main() -> Result<()> {
    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new()
        .with_title("Solid Red")
        .with_inner_size(PhysicalSize::new(800, 600));

    let windowed_context = ContextBuilder::new()
        .with_srgb(true)
        .build_windowed(window_builder, &event_loop)?;

    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    gl::load_with(|symbol| windowed_context.get_proc_address(symbol) as *const _);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::Resized(physical_size) => {
                    windowed_context.resize(*physical_size);
                }

                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: keystate,
                            virtual_keycode: Some(keycode),
                            ..
                        },
                    ..
                } => {
                    if let (VirtualKeyCode::Escape, ElementState::Pressed) = (keycode, keystate) {
                        *control_flow = ControlFlow::Exit;
                    }
                }
                _ => (),
            },
            Event::MainEventsCleared => {
                let red: [GLfloat; 4] = [1.0, 0.0, 0.0, 0.0];
                unsafe {
                    gl::ClearBufferfv(gl::COLOR, 0, &red as *const f32);
                }
                windowed_context.swap_buffers().unwrap();
            }
            _ => (),
        }
    });
}
