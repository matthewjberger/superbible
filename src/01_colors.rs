use support::app::*;

struct DemoApp {
    settings: AppSettings,
}

impl DemoApp {
    pub fn new() -> DemoApp {
        DemoApp {
            settings: AppSettings {
                title: "Fading Colors".to_string(),
                ..Default::default()
            },
        }
    }
}

impl App for DemoApp {
    fn settings(&mut self) -> &AppSettings {
        &self.settings
    }

    fn render(&mut self, current_time: f32) {
        let color: [GLfloat; 4] = [
            (current_time.sin() * 0.5) + 0.5,
            (current_time.cos() * 0.5) + 0.5,
            0.0,
            1.0,
        ];

        unsafe {
            gl::ClearBufferfv(gl::COLOR, 0, &color as *const f32);
        }
    }
}

fn main() {
    DemoApp::new().run();
}
