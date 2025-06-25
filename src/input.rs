use winit::event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent, DeviceEvent, MouseButton};

pub struct Input {
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub quit: bool,
    pub toggle_inventory: bool,
    pub mouse_delta: (f32, f32),
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
            toggle_inventory: false,
            mouse_delta: (0.0, 0.0),
        }
    }

    pub fn process_window_events(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { input: KeyboardInput { virtual_keycode, state, .. }, .. } => {
                if let Some(keycode) = virtual_keycode {
                    let pressed = *state == ElementState::Pressed;
                    match keycode {
                        VirtualKeyCode::W => self.forward = pressed,
                        VirtualKeyCode::S => self.backward = pressed,
                        VirtualKeyCode::A => self.left = pressed,
                        VirtualKeyCode::D => self.right = pressed,
                        VirtualKeyCode::Space => self.up = pressed,
                        VirtualKeyCode::LShift => self.down = pressed,
                        VirtualKeyCode::Escape => self.quit = pressed,
                        VirtualKeyCode::Tab => self.toggle_inventory = pressed,
                        _ => {}
                    }
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if *state == ElementState::Pressed {
                    match button {
                        MouseButton::Left => {}, // Reserve for future
                        MouseButton::Right => {}, // Reserve for future
                        _ => {},
                    }
                }
            }
            _ => {}
        }
    }

    pub fn process_device_events(&mut self, event: &DeviceEvent) {
        if let DeviceEvent::MouseMotion { delta } = event {
            self.mouse_delta = (delta.0 as f32, delta.1 as f32);
        }
    }

    pub fn clear_mouse_delta(&mut self) {
        self.mouse_delta = (0.0, 0.0);
    }
}