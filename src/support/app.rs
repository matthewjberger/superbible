use std::time::Instant;

use anyhow::Result;
use glutin::{
    dpi::PhysicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
    ContextBuilder, ContextWrapper, PossiblyCurrent,
};

pub trait App {
    fn initialize(&mut self, _window: &Window) -> Result<()> {
        Ok(())
    }
    fn update(&mut self) -> Result<()> {
        Ok(())
    }
    fn render(&mut self, _time: f32) -> Result<()> {
        Ok(())
    }
    fn cleanup(&mut self) -> Result<()> {
        Ok(())
    }
    fn on_key(&mut self, _keycode: &VirtualKeyCode, _keystate: &ElementState) -> Result<()> {
        Ok(())
    }
    fn handle_events(&mut self, _event: Event<()>, _window: &Window) -> Result<()> {
        Ok(())
    }
    fn on_resize(&mut self, _width: u32, _height: u32) -> Result<()> {
        Ok(())
    }
}

pub fn run_application(mut app: impl App + 'static, title: &str) -> Result<()> {
    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new()
        .with_title(title)
        .with_inner_size(PhysicalSize::new(800, 600));

    let windowed_context = ContextBuilder::new()
        .with_srgb(true)
        .build_windowed(window_builder, &event_loop)?;

    let context = unsafe { windowed_context.make_current().unwrap() };

    gl::load_with(|symbol| context.get_proc_address(symbol) as *const _);

    let start_time = Instant::now();

    app.initialize(context.window())?;

    event_loop.run(move |event, _, control_flow| {
        if let Err(error) = run_loop(&mut app, &context, event, control_flow, &start_time) {
            eprintln!("Application Error: {}", error);
        }
    });
}

fn run_loop(
    app: &mut impl App,
    context: &ContextWrapper<PossiblyCurrent, Window>,
    event: Event<()>,
    control_flow: &mut ControlFlow,
    start_time: &Instant,
) -> Result<()> {
    *control_flow = ControlFlow::Poll;

    match event {
        Event::LoopDestroyed => app.cleanup()?,
        Event::WindowEvent { ref event, .. } => match event {
            WindowEvent::Resized(physical_size) => {
                context.resize(*physical_size);
                unsafe {
                    gl::Viewport(0, 0, physical_size.width as _, physical_size.height as _);
                }
                app.on_resize(physical_size.width, physical_size.height)?;
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
                app.on_key(keycode, keystate)?;
            }
            _ => (),
        },
        Event::MainEventsCleared => {
            app.update()?;
            app.render(start_time.elapsed().as_secs_f32())?;
            context.swap_buffers()?;
        }
        _ => (),
    }

    app.handle_events(event, context.window())?;

    Ok(())
}
