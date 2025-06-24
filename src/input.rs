use winit::event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent};

pub struct Input {
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub quit: bool,
}

impl Input {
    pub fn new() -> Self {
        Self {
            forward: false,
            backward: false,
            left: false,
            right: false,
            up: false,
            down: false,
            quit: false,
        }
    }

    pub fn process_events(&mut self, event: &WindowEvent) -> bool {
        if let WindowEvent::KeyboardInput { input, .. } = event {
            if let Some(keycode) = input.virtual_keycode {
                let pressed = input.state == ElementState::Pressed;
                match keycode {
                    VirtualKeyCode::W => self.forward = pressed,
                    VirtualKeyCode::S => self.backward = pressed,
                    VirtualKeyCode::A => self.left = pressed,
                    VirtualKeyCode::D => self.right = pressed,
                    VirtualKeyCode::Space => self.up = pressed,
                    VirtualKeyCode::LShift => self.down = pressed,
                    VirtualKeyCode::Escape => self.quit = pressed,
                    _ => {}
                }
            }
            true
        } else {
            false
        }
    }
}
