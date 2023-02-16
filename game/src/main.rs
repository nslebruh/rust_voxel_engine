extern crate gl;

mod data;
mod noise;
mod chunk;
mod block;
mod world;

use std::{mem::size_of, ffi::c_void, path::Path};

use engine::{
    window::Window,
    camera::Camera,
    glfw::*,
    keybinds::{
        InputFunctionArguments, 
        KeyBinding
    },
    input_functions::*,
    shader::Shader,
    glm::{self, vec3}, na::Unit,
};
use block_mesh::{ndshape::{ConstShape3u32, ConstShape}, GreedyQuadsBuffer, greedy_quads, RIGHT_HANDED_Y_UP_CONFIG};

use block::{FULL, EMPTY};
use ::noise::{Fbm, Perlin, Seedable, SuperSimplex, utils::{NoiseMap, PlaneMapBuilder, NoiseMapBuilder}};

use crate::chunk::Chunk;
pub use crate::world::World;

fn main() {
    let scr_width: u32 = 1280;
    let scr_height: u32 = 720;
    let mut camera = Camera::default();
    let mut first_mouse = true;
    let mut last_x: f32 = scr_width as f32 / 2.0;
    let mut last_y: f32 = scr_height as f32 / 2.0;
    let mut delta_time: f32;
    let mut last_frame: f32 = 0.0;

    let img = image::open(&Path::new("dirt.png")).unwrap().to_rgba8();
    let img_data = img.to_vec();

    let mut keybindings: Vec<KeyBinding> = vec![
        KeyBinding::new(Key::Escape, false, set_window_should_close),
        KeyBinding::new(Key::W, true, camera_forward),
        KeyBinding::new(Key::A, true, camera_left),
        KeyBinding::new(Key::S, true, camera_backward),
        KeyBinding::new(Key::D, true, camera_right),
        KeyBinding::new(Key::Space, true, camera_up),
        KeyBinding::new(Key::LeftShift, true, camera_down),
        KeyBinding::new(Key::RightShift, false, toggle_cursor_mode),
        KeyBinding::new(Key::RightControl, false, print_camera_pos),
        KeyBinding::new(Key::LeftControl, false, increase_movement_speed),
        KeyBinding::new(Key::F11, false, toggle_fullscreen)
    ];

    // A 16^3 chunk with 1-voxel boundary padding.
    pub type ChunkShape = ConstShape3u32<18, 18, 18>;

    // This chunk will cover just a single octant of a sphere SDF (radius 15).
    let mut voxels = [EMPTY; ChunkShape::SIZE as usize];
    for i in 0..ChunkShape::SIZE {
        let [x, y, z] = ChunkShape::delinearize(i);
        voxels[i as usize] = if ((x * x + y * y + z * z) as f32).sqrt() < 15.0 {
            FULL
        } else {
            EMPTY
        };
    }
    let faces = RIGHT_HANDED_Y_UP_CONFIG.faces;

    let mut buffer = GreedyQuadsBuffer::new(voxels.len());
    greedy_quads(
        &voxels,
        &ChunkShape {},
        [0; 3],
        [17; 3],
        &faces,
        &mut buffer
    );
    println!("{}", buffer.quads.num_quads());

    // Some quads were generated.
    assert!(buffer.quads.num_quads() > 0);
    let num_indices = buffer.quads.num_quads() * 6;
    let num_vertices = buffer.quads.num_quads() * 4;
    let mut indices = Vec::with_capacity(num_indices);
    let mut positions = Vec::with_capacity(num_vertices);
    let mut normals = Vec::with_capacity(num_vertices);
    let mut tex_coords = Vec::with_capacity(num_vertices);
    for (group, face) in buffer.quads.groups.into_iter().zip(faces.into_iter()) {
        for quad in group.into_iter() {
            indices.extend_from_slice(&face.quad_mesh_indices(positions.len() as u32));
            positions.extend_from_slice(&face.quad_mesh_positions(&quad, 1.0));
            normals.extend_from_slice(&face.quad_mesh_normals());
            tex_coords.extend_from_slice(&face.tex_coords(
                RIGHT_HANDED_Y_UP_CONFIG.u_flip_face,
                true,
                &quad,
            ));
        }
    }
    let mut test_vertices: Vec<f32> = Vec::with_capacity(num_indices * 5);
    for index in indices.into_iter() {
        test_vertices.extend_from_slice(&positions[index as usize]);
        test_vertices.extend_from_slice(&tex_coords[index as usize])
    }

    println!("test_vertices.len(), {}", test_vertices.len());

    //let super_simplex = SuperSimplex::default();
    //write_example_to_file(
    //    &PlaneMapBuilder::<_, 2>::new(super_simplex).build(),
    //    "super_simplex.png",
    //);

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
    
    let world = World::new(0,  2);

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }
    window.make_current();
    window.set_cursor_pos_polling(true);
    window.set_framebuffer_size_polling(true);
    window.set_scroll_polling(true);
    window.set_cursor_mode(CursorMode::Disabled);

    //let noise = Fbm::<Perlin>::default().set_seed(0);
    //let chunk = Chunk::new(vec3(0, 0, 0), &noise);

    let mut vao: u32 = 0;
    let mut vbo: u32 = 0;
    // mut ebo: u32 = 0;
    let mut texture: u32 = 0;



    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        //gl::GenBuffers(1, &mut ebo);

        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (test_vertices.len() * size_of::<gl::types::GLfloat>()) as gl::types::GLsizeiptr,
            test_vertices.as_ptr().cast(),
            gl::STATIC_DRAW
        );

        //gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        //gl::BufferData(
        //    gl::ELEMENT_ARRAY_BUFFER,
        //    (indices.len() * size_of::<u32>()) as isize,
        //    indices.as_ptr().cast(),
        //    gl::STATIC_DRAW
        //);

        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (5 * size_of::<f32>()).try_into().unwrap(),
            std::ptr::null(),
        );

        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            (5 * size_of::<f32>()).try_into().unwrap(),
            (3 * size_of::<f32>()) as *const c_void,
        );

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);

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

    while !window.should_close() {
        let current_frame = window.context.get_time() as f32;
        delta_time = current_frame - last_frame;
        last_frame = current_frame;

        process_input(&mut window, &mut delta_time, &mut keybindings, &mut camera);
        window.process_events(&mut first_mouse, &mut last_x, &mut last_y, &mut camera);

        unsafe {
            gl::ClearColor(0.5, 0.5, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        };

        let (width, height) = window.get_framebuffer_size();

        let model = glm::Mat4::from_axis_angle(&Unit::new_normalize(vec3(0.0, 1.0, 0.0)), 90.0);
        let projection = glm::perspective_fov(1.0, width as f32, height as f32, 0.1, 100.0);
        let view = camera.get_view_matrix();

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

            for chunk in world.chunks.iter() {
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
        binding.update(action, InputFunctionArguments::new().camera(camera).window(window).delta_time(delta_time).action(&action))
    }
}
pub fn write_example_to_file(map: &NoiseMap, filename: &str) {
    map.write_to_file(filename)
}