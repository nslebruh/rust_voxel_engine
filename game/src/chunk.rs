use std::collections::HashSet;
use std::ffi::c_void;
use std::mem::size_of;

use block_mesh::{RIGHT_HANDED_Y_UP_CONFIG, greedy_quads, GreedyQuadsBuffer};
use block_mesh::ndshape::{ConstShape, ConstShape2u32};
use engine::glm;
use engine::shader::Shader;
use noise::{Fbm, Perlin, NoiseFn};

use crate::block::Block;
use crate::glm::{I32Vec3, Vec3};
use crate::ConstShape3u32;

pub type ChunkSize = ConstShape3u32<18, 18, 18>;
pub type NoiseSize = ConstShape2u32<16, 16>;

#[derive(Debug, Clone)]
pub struct Chunk {
    pub filled_blocks: HashSet<(u32, u32, u32)>,
    pub position: I32Vec3,
    pub blocks: [Block; 5832],
    pub has_changed: bool,
    is_empty: bool,
    visible: bool,
    mesh: Option<Vec<f32>>,
    vao: Option<u32>,
    vbo: Option<u32>
}

impl Chunk {
    pub fn new(position: I32Vec3, noise: &Fbm<Perlin>) -> Self {
        println!("{}", position.y);
        let x_offset = position.x * 16;
        let y_offset = position.y as u32 * 16;
        let z_offset = position.z * 16;
        let mut noise_vec: [u32; 256] = [0; 256];
        let mut pos_full: u32 = 0;
        let mut pos_empty: u32 = 0;
        let mut filled_blocks = HashSet::new();

        let visible;
        let is_empty;
        let mut blocks;


        // Check bottom y slice for air, if full of air, chunk empty
        // Check top y slice for block, if full of block, chunk full
        for x in 0..16 {
            for z in 0..16 {
                let noise_val = ((noise.get([(x as i32 + x_offset) as f64 / 200.0, (z as i32 + z_offset) as f64 / 200.0]) * 0.5 + 0.5).clamp(0.0, 1.0) * 128.0) as u32 + 128;
                noise_vec[NoiseSize::linearize([x, z]) as usize] = noise_val;
                if y_offset >= noise_val {
                    pos_empty += 1
                } else if y_offset + 15 <= noise_val {
                    pos_full += 1
                }
            }
        }
        println!("pos_empty: {}, pos_full: {}", pos_empty, pos_full);
        if pos_empty == 256 {
            visible = false;
            blocks = [Block::AIR; 5832];
            is_empty = true;
        } else if pos_full == 256 {
            visible = true;
            is_empty = false;
            blocks = [Block::AIR; 5832];
            for i in 0..ChunkSize::SIZE {
                let [x, y, z] = ChunkSize::delinearize(i);
                if (x > 0 && x < 17)  && (y > 0 && y < 17) && (z > 0 && z < 17) {
                    filled_blocks.insert((x - 1, y - 1, z - 1));
                    blocks[i as usize] = Block::STONE
                }
            }
        } else {
            visible = true;
            is_empty = false;
            blocks = [Block::AIR; 5832];
            for i in 0..ChunkSize::SIZE {
                let [x, y, z] = ChunkSize::delinearize(i);
                if (x > 0 && x < 17)  && (y > 0 && y < 17) && (z > 0 && z < 17) {
                    let noise_val = noise_vec[NoiseSize::linearize([x - 1, z - 1]) as usize];
                    if y + y_offset <= noise_val {
                        filled_blocks.insert((x - 1, y - 1, z - 1));
                        blocks[i as usize] = Block::DIRT
                    }
                }
            }
        }

        Self {
            blocks,
            filled_blocks,
            is_empty,
            has_changed: true,
            mesh: None,
            position,
            visible,
            vao: None,
            vbo: None,
        }
    }

    pub fn gen_from_heightmap(position_y: i32, noise_vec: [u32; 256]) -> [Block; 5832]{
        let mut blocks = [Block::AIR; 5832];
        let y_offset = position_y as u32 * 16;
        for i in 0..ChunkSize::SIZE {
            let [x, y, z] = ChunkSize::delinearize(i);
            if (x > 0 && x < 17)  && (y > 0 && y < 17) && (z > 0 && z < 17) {
                let noise_val = noise_vec[NoiseSize::linearize([x - 1, z - 1]) as usize];
                if y + y_offset <= noise_val {
                    blocks[i as usize] = Block::DIRT
                }
            }
        }
        blocks
    }

    pub fn update_block(&mut self, position: I32Vec3, block: Block) {
        self.blocks[ChunkSize::linearize([position.x as u32, position.y as u32, position.z as u32]) as usize] = block;
    }

    pub fn create_mesh(&mut self) {
        if self.is_empty {
            return
        }
        let blocks = self.blocks;
        let faces = RIGHT_HANDED_Y_UP_CONFIG.faces;

        let mut buffer = GreedyQuadsBuffer::new(blocks.len());
        greedy_quads(
            &blocks,
            &ChunkSize {},
            [0; 3],
            [17; 3],
            &faces,
            &mut buffer
        );
        println!("quads: {}", buffer.quads.num_quads());
        if buffer.quads.num_quads() == 0 {
            self.visible = false;
        }

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
            test_vertices.extend_from_slice(&normals[index as usize]);
            test_vertices.extend_from_slice(&tex_coords[index as usize])
        }
        println!("test_vertices.len(), {}", test_vertices.len());
        let (vertices, vao, vbo) = unsafe {
            Chunk::setup_mesh(test_vertices)
        };
        self.mesh = Some(vertices);
        self.vao = Some(vao);
        self.vbo = Some(vbo);
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

        gl::EnableVertexAttribArray(0); // position coords
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (8 * size_of::<f32>()).try_into().unwrap(),
            std::ptr::null(),
        );

        gl::EnableVertexAttribArray(1); // normal coords
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            (8 * size_of::<f32>()).try_into().unwrap(),
            (3 * size_of::<f32>()) as *const c_void,
        );

        gl::EnableVertexAttribArray(2); // texture coords
        gl::VertexAttribPointer(
            2,
            2,
            gl::FLOAT,
            gl::FALSE,
            (8 * size_of::<f32>()).try_into().unwrap(),
            (6 * size_of::<f32>()) as *const c_void,
        );

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);

        (data, vao, vbo)
    }

    pub unsafe fn draw(&self, texture: &u32, shader: &Shader) {
        if !self.visible {
            return
        }
        gl::BindVertexArray(self.vao.unwrap());
        gl::BindTexture(gl::TEXTURE_2D, *texture);
        shader.use_program();
        let model = glm::translation(&Vec3::new(((self.position.x * 16) - 1) as f32, ((self.position.y * 16) - 1 /* size of cube gen */) as f32, ((self.position.z * 16) - 1) as f32,));
        shader.set_mat4("model", &model);
        gl::DrawArrays(gl::TRIANGLES, 0, (self.mesh.as_ref().unwrap().len() / 3) as i32);
        gl::BindVertexArray(0);
        gl::BindTexture(gl::TEXTURE_2D, 0);

    }
}