use std::ffi::c_void;
use std::mem::size_of;

use block_mesh::{RIGHT_HANDED_Y_UP_CONFIG, greedy_quads, GreedyQuadsBuffer};
use block_mesh::ndshape::ConstShape;
use engine::glm;
use engine::shader::Shader;
use noise::{Fbm, Perlin, NoiseFn};

use crate::block::Block;
use crate::glm::{I32Vec3, Vec3};
use crate::ConstShape3u32;

pub type ChunkSize = ConstShape3u32<18, 18, 18>;

#[derive(Debug)]
pub struct Chunk {
    position: I32Vec3,
    blocks: [Block; 5832],
    mesh: Vec<f32>,
    vao: u32,
    vbo: u32
}

impl Chunk {
    pub fn new(position: I32Vec3, noise: &Fbm<Perlin>) -> Self {
        println!("{}", position.y);
        let x_offset = position.x * 16;
        let y_offset = position.y * 16;
        let z_offset = position.z * 16;
        let mut blocks = [Block::default(); ChunkSize::SIZE as usize];
        for i in 0..ChunkSize::SIZE {
            let [x, y, z] = ChunkSize::delinearize(i);
            if (x > 0 && x < 16)  && (y > 0 && y < 16) && (z > 0 && z < 16) {
                let noise_val = ((noise.get([(x as i32 + x_offset) as f64 / 25.0, (z as i32 + z_offset) as f64 / 25.0]) * 0.5 + 0.5).clamp(0.0, 1.0) * 255.0).round() as u32;
                if y + (y_offset as u32) <= noise_val / 2{
                    blocks[i as usize] = Block::STONE
                }
            }
        }
        let (mesh, vao, vbo) = Chunk::create_mesh(&blocks);

        Self {
            blocks,
            mesh,
            position,
            vao,
            vbo
        }
    }

    fn create_mesh(voxels: &[Block; 5832]) -> (Vec<f32>, u32, u32) {
        let faces = RIGHT_HANDED_Y_UP_CONFIG.faces;

        let mut buffer = GreedyQuadsBuffer::new(voxels.len());
        greedy_quads(
            voxels,
            &ChunkSize {},
            [0; 3],
            [17; 3],
            &faces,
            &mut buffer
        );
        println!("{}", buffer.quads.num_quads());
        if buffer.quads.num_quads() == 0 {return (Vec::new(), 0, 0)}
        // Some quads were generated.
        //assert!(buffer.quads.num_quads() > 0);

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
        unsafe {
            Chunk::setup_mesh(test_vertices)
        }
    }

    unsafe fn setup_mesh(data: Vec<f32>) -> (Vec<f32>, u32, u32) {
        let mut vao: u32 = 0;
        let mut vbo: u32 = 0;

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);

        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (data.len() * size_of::<gl::types::GLfloat>()) as gl::types::GLsizeiptr,
            data.as_ptr().cast(),
            gl::STATIC_DRAW
        );

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

        (data, vao, vbo)
    }

    pub unsafe fn draw(&self, texture: &u32, shader: &Shader) {
        if self.mesh.len() == 0 {
            return
        }
        gl::BindVertexArray(self.vao);
        gl::BindTexture(gl::TEXTURE_2D, *texture);
        shader.use_program();
        let model = glm::translation(&Vec3::new(((self.position.x * 15) - 1) as f32, ((self.position.y * 15) + 8 /* size of cube gen */) as f32, ((self.position.z * 15) - 1) as f32,));
        shader.set_mat4("model", &model);
        gl::DrawArrays(gl::TRIANGLES, 0, (self.mesh.len() / 3) as i32);
        gl::BindVertexArray(0);
        gl::BindTexture(gl::TEXTURE_2D, 0);

    }
}

//pub fn chunk_pos_to_f32(pos: I32Vec3) -> Vec3 {
//    Vec3::new(pos.x as f32, pos.y as f32, pos.z as f32)
//}