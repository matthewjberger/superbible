use anyhow::Result;
use gl::types::*;
use support::{app::run_application, app::App};

#[derive(Default)]
struct DemoApp;

impl App for DemoApp {
    fn render(&mut self, time: f32) -> Result<()> {
        let color: [GLfloat; 4] = [(time.sin() * 0.5) + 0.5, (time.cos() * 0.5) + 0.5, 0.0, 1.0];
        unsafe {
            gl::ClearBufferfv(gl::COLOR, 0, &color as *const f32);
        }

        Ok(())
    }
}

fn main() -> Result<()> {
    let app = DemoApp::default();
    run_application(app, "Fading Colors")
}
