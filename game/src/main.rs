extern crate gl;

pub mod noise;
pub mod chunk;
pub mod block;
pub mod world;
pub mod player;

use std::path::Path;

use engine::{
    window::Window,
    camera::{Camera, CameraMovement},
    glfw::*,
    keybinds::{
        InputFunctionArguments, 
        KeyBinding
    },
    input_functions::*,
    shader::Shader,
    glm::{
        self,
        vec3,
        IVec3, Vec3
    },
    na::{Unit, OPoint, Const, Point3},
};
use block_mesh::ndshape::ConstShape3u32;


use crate::player::Player;
pub use crate::world::World;

pub struct Game {
    pub world: World,
    pub player_is_colliding: bool,
    pub test_collision: bool
}

impl Game {
    pub fn new(world: World) -> Self {
        Self {
            world,
            player_is_colliding: false,
            test_collision: true
        }
    }

    pub fn run_loop(&mut self) {
        self.set_floored_position();
        self.set_current_chunk();
        self.detect_collision();
    } 

    pub fn set_current_chunk(&mut self) {
        let current_position = IVec3::new(self.world.player.camera.position.x.floor() as i32, self.world.player.camera.position.y.floor() as i32, self.world.player.camera.position.z.floor() as i32);
        let mut current_chunk = IVec3::new(current_position.x / 16, current_position.y / 16, current_position.z / 16);
        if current_position.x.is_negative() {
            current_chunk.x = current_chunk.x - 1;
        }
        if current_position.y.is_negative() {
            current_chunk.y = current_chunk.y - 1;
        }
        if current_position.z.is_negative() {
            current_chunk.z = current_chunk.z - 1;
        }
        self.world.current_chunk = current_chunk;
      }
    
    pub fn set_floored_position(&mut self) {
        let floored_position = IVec3::new(self.world.player.camera.position.x.floor() as i32, self.world.player.camera.position.y.floor() as i32, self.world.player.camera.position.z.floor() as i32);
        let mut normalised_floored_position = [floored_position.x % 16, floored_position.y % 16, floored_position.z % 16];
        for pos in normalised_floored_position.iter_mut() {
            if pos.is_negative() {
                *pos = *pos + 16
            }
        }
        self.world.player.floored_normal_position = (normalised_floored_position[0] as u32, normalised_floored_position[1] as u32, normalised_floored_position[2] as u32);
    }
    
    
    pub fn detect_collision(&mut self) {
        if self.world.chunk_positions.contains(&self.world.current_chunk) {
            if self.world.chunks[self.world.calc_chunk_index(self.world.current_chunk)].filled_blocks.contains(&self.world.player.floored_normal_position) {
                //println!("Player colliding at {:?} in chunk {:?}", self.world.player.floored_normal_position, self.world.current_chunk);
                //println!("Get outta there :(");
                self.player_is_colliding = true;
              } else {
                self.player_is_colliding = false;
              }
            }
    }

    pub fn move_player(&mut self, position: OPoint<f32, Const<3>>) {
        let fixed_position = Vec3::new(position.x - 0.15, position.y - 0.15, position.z - 0.15); // needs fixing - edge cases
        let floored_position = IVec3::new(fixed_position.x.floor() as i32, fixed_position.y.floor() as i32, fixed_position.z.floor() as i32);
        let mut current_chunk = IVec3::new(floored_position.x / 16, floored_position.y / 16, floored_position.z / 16);
        if floored_position.x.is_negative() {
            current_chunk.x = current_chunk.x - 1;
        }
        if floored_position.y.is_negative() {
            current_chunk.y = current_chunk.y - 1;
        }
        if floored_position.z.is_negative() {
            current_chunk.z = current_chunk.z - 1;
        }
        let mut normalised_floored_position = [floored_position.x % 16, floored_position.y % 16, floored_position.z % 16];
        for pos in normalised_floored_position.iter_mut() {
            if pos.is_negative() {
                *pos = *pos + 16
            }
        }
        let floored_normal_position = (normalised_floored_position[0] as u32, normalised_floored_position[1] as u32, normalised_floored_position[2] as u32);
        if self.test_collision {
            if self.world.chunk_positions.contains(&current_chunk) {
                if self.world.chunks[self.world.calc_chunk_index(current_chunk)].filled_blocks.contains(&floored_normal_position) {
                    println!("simulated position in a block")
                } else {
                    self.world.player.camera.position = position;
                } 
                //self.world.player.camera.position = position;
            } else {
                self.world.player.camera.position = position;
            };
        } else {
            self.world.player.camera.position = position;
        }
        
    }

    fn process_movement_input(&mut self, window: &mut Window, delta_time: &f32, bindings: &mut [(Key, CameraMovement)]) {
        let mut vector: Vec<CameraMovement> = Vec::new();
        for binding in bindings.iter_mut() {
            if window.get_key(binding.0) == Action::Press {
                vector.push(binding.1)
            }
        }
        if vector.len() != 0 {
            println!("{:?}", vector);
            let position = self.world.player.camera.process_input_vector(vector, delta_time);
            self.move_player(position);
        }
    }
}

fn main() {
    let scr_width: u32 = 1280;
    let scr_height: u32 = 720;
    let camera = Camera {
        position: Point3::<f32>::new(1.0, 257.0, 1.0),
        ..Default::default()
    };
    let mut first_mouse = true;
    let mut last_x: f32 = scr_width as f32 / 2.0;
    let mut last_y: f32 = scr_height as f32 / 2.0;
    let mut delta_time: f32;
    let mut last_frame: f32 = 0.0;

    let img = image::open(&Path::new("dirt.png")).unwrap().to_rgba8();
    let img_data = img.to_vec();

    let mut input_keybindings:  Vec<(Key, CameraMovement)> = vec![
        (Key::W, CameraMovement::Forward),
        (Key::A, CameraMovement::Left),
        (Key::S, CameraMovement::Backward),
        (Key::D, CameraMovement::Right),
        (Key::Space, CameraMovement::Up),
        (Key::LeftShift, CameraMovement::Down)

    ];

    let mut keybindings: Vec<KeyBinding> = vec![
        KeyBinding::new(Key::Escape, false, set_window_should_close),
        //KeyBinding::new(Key::W, true, camera_forward),
        //KeyBinding::new(Key::A, true, camera_left),
        //KeyBinding::new(Key::S, true, camera_backward),
        //KeyBinding::new(Key::D, true, camera_right),
        //KeyBinding::new(Key::Space, true, camera_up),
        //KeyBinding::new(Key::LeftShift, true, camera_down),
        KeyBinding::new(Key::RightShift, false, toggle_cursor_mode),
        KeyBinding::new(Key::RightControl, false, print_camera_pos),
        KeyBinding::new(Key::LeftControl, false, increase_movement_speed),
        KeyBinding::new(Key::F11, false, toggle_fullscreen)
    ];

    let mut window = Window::init(
        1280,
        720,
        "test",
        WindowMode::Windowed,
        vec![
            WindowHint::ContextVersion(3, 3),
            WindowHint::OpenGlProfile(OpenGlProfileHint::Core)
            ]
        ).unwrap();

    gl::load_with(|ptr| window.get_proc_address(ptr) as *const _);
    
    let world = World::new(0, 16, false, Player::new(camera));

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        //gl::Enable(gl::CULL_FACE);
    }
    window.make_current();
    window.set_cursor_pos_polling(true);
    window.set_framebuffer_size_polling(true);
    window.set_scroll_polling(true);
    window.set_cursor_mode(CursorMode::Disabled);

    // mut ebo: u32 = 0;
    let mut texture: u32 = 0;

    unsafe {
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as i32,
            img.width() as i32,
            img.height() as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            &img_data[0] as *const u8 as *const std::ffi::c_void
        );
        gl::GenerateMipmap(gl::TEXTURE_2D)

    }

    //unsafe {
    //    gl::BindVertexArray(vao);
    //    gl::BindTexture(gl::TEXTURE_2D, texture);
    //}

    let shader_program = Shader::new("triangle.vert", "triangle.frag");

    let mut game = Game::new(world);

    while !window.should_close() {
        let current_frame = window.context.get_time() as f32;
        delta_time = current_frame - last_frame;
        last_frame = current_frame;

        process_input(&mut window, &mut delta_time, &mut keybindings, &mut game.world.player.camera);
        window.process_events(&mut first_mouse, &mut last_x, &mut last_y, &mut game.world.player.camera);
        if window.get_key(Key::Backslash) == Action::Press {
            game.test_collision = false;
        } else {
            game.test_collision = true;
        }
        game.run_loop();
        game.process_movement_input(&mut window, &delta_time, &mut input_keybindings);
        

        unsafe {
            gl::ClearColor(0.5, 0.5, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        };

        let (width, height) = window.get_framebuffer_size();

        let model = glm::Mat4::from_axis_angle(&Unit::new_normalize(vec3(0.0, 1.0, 0.0)), 90.0);
        let projection = glm::perspective_fov(1.0, width as f32, height as f32, 0.1, 100.0);
        let view = game.world.player.camera.get_view_matrix();

        unsafe {
            shader_program.set_mat4("model", &model);
            shader_program.set_mat4("view", &view);
            shader_program.set_mat4("projection", &projection);
            shader_program.set_vec4("ourColor", 1.0, 1.0, 1.0, 0.0);
        }

        unsafe {
            //shader_program.use_program();
            //gl::DrawArrays(gl::TRIANGLES, 0, (test_vertices.len() / 3) as i32);

            //chunk.draw(&texture, &shader_program)

            for chunk in game.world.chunks.iter() {
                chunk.draw(&texture, &shader_program)
            }
        }

        window.swap_buffers();
        window.poll_events();
    };
    println!("Hello world!");
}

fn process_input(window: &mut Window, delta_time: &f32, bindings: &mut [KeyBinding], camera: &mut Camera) {
    for binding in bindings.iter_mut() {
        let action = window.get_key(binding.key);
        binding.update(action, InputFunctionArguments::new().camera(camera).window(window).delta_time(delta_time).action(action.clone()))
    }
}

