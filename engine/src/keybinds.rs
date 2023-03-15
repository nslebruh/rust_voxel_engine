use glfw::{Key, Action};
use super::window::Window;

use super::camera::Camera;

pub struct KeyBinding {
    pub key: Key,
    pub state: bool,
    pub callback: fn(InputFunctionArguments),
    run_every_frame: bool
}

impl KeyBinding {
    pub fn new(key: Key, run_every_frame: bool, callback: fn(InputFunctionArguments)) -> KeyBinding {
        KeyBinding {
            key,
            state: false,
            callback,
            run_every_frame
        }
    }

    pub fn update(&mut self, action: Action, args: InputFunctionArguments) {
            if self.run_every_frame {
                if action == Action::Press {
                    (self.callback)(args)
                }
            } else {
                match action {
                    Action::Press if !self.state => {
                        self.state = true;
                        (self.callback)(args);
                    },
                    Action::Release if self.state => {
                        self.state = false;
                        (self.callback)(args);
                    },
                    Action::Release => {
                        self.state = false;
                    },
                    _ => {}
                }
            }
    }

}
pub struct InputFunctionArguments<'a> {
    pub window: Option<&'a mut Window>,
    pub camera: Option<&'a mut Camera>,
    pub delta_time: Option<&'a f32>,
    pub action: Option<Action>,
}

impl<'a> InputFunctionArguments<'a> {
    pub fn new() -> Self {
        Self {
            window: None,
            camera: None,
            delta_time: None,
            action: None
        }
    }

    pub fn window(self, window: &'a mut Window) -> Self {
        Self {
            window: Some(window),
            ..self
        }
    }

    pub fn camera(self, camera: &'a mut Camera) -> Self {
        Self {
            camera: Some(camera),
            ..self
        }
    }
    
    pub fn delta_time(self, delta_time: &'a f32) -> Self {
        Self {
            delta_time: Some(delta_time),
            ..self
        }
    }

    pub fn action(self, action: Action) -> Self {
        Self {
            action: Some(action),
            ..self
        }
    } 
}