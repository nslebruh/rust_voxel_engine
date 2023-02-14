use block_mesh::{RIGHT_HANDED_Y_UP_CONFIG, greedy_quads, GreedyQuadsBuffer};
use block_mesh::ndshape::ConstShape;
use noise::{Fbm, Perlin};

use crate::block::Block;
use crate::glm::I32Vec3;
use crate::ConstShape3u32;

type ChunkSize = ConstShape3u32<16, 16, 16>;

#[derive(Debug, Default)]
pub struct Chunk {
    position: I32Vec3,
    blocks: Vec<Block>,
    mesh: Vec<f32>,
    mesh_vao: u32,
    mesh_vbo: u32
}

impl Chunk {
    pub fn new(position: I32Vec3, noise: &Fbm<Perlin>) -> Self {
        let x_offset = position.x * 16;
        let y_offset = position.y * 16;
        let z_offset = position.z * 16;
        let mut blocks = vec![Block::default(); ChunkSize::SIZE as usize];
        let mut mesh = Chunk::create_mesh(&blocks);

        Self {
            blocks,
            mesh,
            position,
            ..Default::default()
        }
    }

    fn create_mesh(voxels: &Vec<Block>) -> Vec<f32> {
        let faces = RIGHT_HANDED_Y_UP_CONFIG.faces;

        let mut buffer = GreedyQuadsBuffer::new(voxels.len());
        greedy_quads(
            &voxels,
            &ChunkSize {},
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
        test_vertices
    }

    unsafe fn setup_mesh() {
        
    }

    pub unsafe fn draw(&self) {

    }
}