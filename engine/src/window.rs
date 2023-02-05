pub struct Window {
    pub context: glfw::Glfw,
    pub window: glfw::Window,
    pub receiver: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
    pub is_fullscreen: bool,
    pub last_pos: (i32, i32),
    pub last_size: (i32, i32)
}

impl Window {
    pub fn init(width: u32, height: u32, title: &str, mode: glfw::WindowMode, hints: Vec<glfw::WindowHint>) -> Self {
        let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        for hint in hints {
            glfw.window_hint(hint)
        }
        let window = glfw.create_window(width, height, title, mode);
        match window {
            Some(window_res) => {
                Self {
                    context: glfw,
                    window: window_res.0,
                    receiver: window_res.1,
                    last_pos: (0, 0),
                    last_size: (width as i32, height as i32),
                    is_fullscreen: match mode {
                        glfw::WindowMode::FullScreen(_) => true,
                        glfw::WindowMode::Windowed => false
                    }
                }
            },
            None => {
                panic!("Unable to create glfw window")
            }
        }
    }

    pub fn process_events(&mut self, first_mouse: &mut bool, last_x: &mut f32, last_y: &mut f32, camera: &mut crate::camera::Camera) {
        for (_, event) in glfw::flush_messages(&self.receiver) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    unsafe { gl::Viewport(0, 0, width, height) }
                },
                glfw::WindowEvent::Pos(xpos, ypos) => {
                    self.last_pos = (xpos, ypos);
                },
                glfw::WindowEvent::Size(width, height) => {
                    self.last_size = (width, height);
                },
                glfw::WindowEvent::CursorPos(x_pos, y_pos) => {
                    let (xpos, ypos) = (x_pos as f32, y_pos as f32);
                    if *first_mouse {
                        *last_x = xpos;
                        *last_y = ypos;
                        *first_mouse = false;
                    }
    
                    let x_offset = xpos - *last_x;
                    let y_offset = *last_y - ypos;
    
                    *last_x = xpos;
                    *last_y = ypos;
                    camera.process_mouse_input(x_offset, y_offset, true)
                },
                glfw::WindowEvent::Scroll(_x_offset, y_offset) => {
                    camera.process_scroll_input(y_offset as f32);
                },
                _ => {}
            }
        }
    }

    pub fn poll_events(&mut self) {
        self.context.poll_events()
    }

    pub fn swap_buffers(&mut self) {
        self.window.swap_buffers();
    }

    #[allow(dead_code)]
    pub fn set_should_close(&mut self, value: bool) {
        self.window.set_should_close(value)
    }

    pub fn make_current(&mut self) {
        self.window.make_current()
    }

    #[allow(dead_code)]
    pub fn toggle_fullscreen(&mut self) {
        if self.is_fullscreen {
                self.set_monitor(
                    glfw::WindowMode::Windowed,
                    self.last_pos.0,
                    self.last_pos.1,
                    self.last_size.0,
                    self.last_size.1,
                    None
                );
            self.is_fullscreen = false;
        } else {
            self.last_pos = self.get_pos();
            self.last_size = self.get_size();

            self.context.with_primary_monitor(|_: &mut glfw::Glfw, m: Option<&glfw::Monitor>| {
                let monitor = m.unwrap();

                let mode = monitor.get_video_mode().unwrap();
                
                self.window.set_monitor(glfw::WindowMode::FullScreen(monitor), 0, 0, mode.width, mode.height, Some(mode.refresh_rate))
                
            });
            self.is_fullscreen = true;

        }
    }

    pub fn get_key(&mut self, key: glfw::Key) -> glfw::Action {
        self.window.get_key(key)
    }

    pub fn set_cursor_mode(&mut self, mode: glfw::CursorMode) {
        self.window.set_cursor_mode(mode)
    }

    pub fn get_framebuffer_size(&mut self) -> (i32, i32) {
        self.window.get_framebuffer_size()
    }

    pub fn set_framebuffer_size_polling(&mut self, val: bool) {
        self.window.set_framebuffer_size_polling(val)
    }
    
    pub fn set_cursor_pos_polling(&mut self, val: bool) {
        self.window.set_cursor_pos_polling(val)
    }
    pub fn set_scroll_polling(&mut self, val: bool) {
        self.window.set_scroll_polling(val)
    }

    pub fn get_proc_address(&mut self, procname: &str) -> glfw::GLProc  {
        self.window.get_proc_address(procname)
    }

    pub fn should_close(&mut self) -> bool {
        self.window.should_close()
    }

    pub fn set_monitor(&mut self, mode: glfw::WindowMode, xpos: i32, ypos: i32, width: i32, height: i32, refresh_rate: Option<u32>) {
        self.window.set_monitor(mode, xpos, ypos, width as u32, height as u32, refresh_rate)
    }

    #[allow(dead_code)]
    pub fn set_pos(&mut self, xpos: i32, ypos: i32) {
        self.window.set_size(xpos, ypos)
    }
    
    #[allow(dead_code)]
    pub fn set_size(&mut self, width: i32, height: i32) {
        self.window.set_size(width, height)
    }

    pub fn get_pos(&mut self) -> (i32, i32) {
        self.window.get_pos()
    }

    pub fn get_size(&mut self) -> (i32, i32) {
        self.window.get_size()
    }
}