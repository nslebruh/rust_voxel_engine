use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

fn benchmark(c: &mut Criterion) {
    let noise: Fbm<Perlin> = Fbm::<Perlin>::default();
    let mut group = c.benchmark_group("gen chunk");
    for x in 0..4 {
        for y in 0..4 {
            for z in 0..4 {
                group.bench_with_input(BenchmarkId::new(vec3(x, y, z).to_string(), vec3(x, y, z)), &vec3(x, y, z), |b, vec3| {b.iter(|| Chunk::new(*vec3, &noise))});

            }
        }
    }
    //for size in [KB, 2 * KB, 4 * KB, 8 * KB, 16 * KB].iter() {
    //    group.throughput(Throughput::Bytes(*size as u64));
    //    group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
    //        b.iter(|| iter::repeat(0u8).take(size).collect::<Vec<_>>());
    //    });
    //}
    group.finish();
    //c.bench_function("create_chunk", |b| b.iter(|| Chunk::new(vec3(0, 0, 0), &noise)));
}

criterion_group!(benches, benchmark);
criterion_main!(benches);

// 2d heightmap for height
use block_mesh::ndshape::{RuntimeShape, Shape};
use noise::{Seedable, MultiFractal};
use crate::glm::vec3;

#[derive(Debug, Default)]
pub struct World {
    //loaded_chunks: Vec<Chunk>,
    //chunks_to_load: Vec<Chunk>,
    pub chunks: Vec<Chunk>,
    noise: Fbm<Perlin>,
    min: (i32, i32, i32)
}

impl World {
    pub fn new(seed: u32, cube_size: u32) -> Self {
        let shape = RuntimeShape::<u32, 3>::new([cube_size; 3]);
        let min_val = -((cube_size % 2) as i32);
        let min = (min_val, min_val, min_val);
        let noise: Fbm<Perlin> = Fbm::<Perlin>::default().set_seed(seed).set_persistence(0.25);
        let mut chunks: Vec<Chunk> = Vec::new();
        for i in 0..shape.size() {
            let [x, y, z] = shape.delinearize(i);
            chunks.push(Chunk::new(vec3(x as i32 + min_val, y as i32, z as i32 + min_val), &noise))
        }
        Self {
            noise,
            chunks,
            min
        }
    }

}

use std::ffi::c_void;
use std::mem::size_of;

use block_mesh::{RIGHT_HANDED_Y_UP_CONFIG, greedy_quads, GreedyQuadsBuffer};
use block_mesh::ndshape::{ConstShape, ConstShape3u32};
use engine::glm;
use engine::shader::Shader;
use noise::{Fbm, Perlin, NoiseFn};

use crate::glm::{I32Vec3, Vec3};

type ChunkSize = ConstShape3u32<18_u32, 18_u32, 18_u32>;

#[derive(Debug)]
pub struct Chunk {
    position: I32Vec3,
    blocks: [Block; 5832],
    is_empty: bool,
    mesh: Vec<f32>,
    vao: u32,
    vbo: u32
}

impl Chunk {
    pub fn new(position: I32Vec3, noise: &Fbm<Perlin>) -> Self {
        println!("{}", position.y);
        let x_offset = position.x * 16;
        let y_offset = position.y as u32 * 16;
        let z_offset = position.z * 16;
        let mut blocks = [Block::default(); ChunkSize::SIZE as usize];
        let mut noise_vec: Vec<Vec<u32>> = vec![Vec::new(); 16];
        let mut pos_filled: u32 = 0;
        let mut pos_empty: u32 = 0;

        for x in 0..16 {
            for z in 0..16 {
                let noise_val = ((noise.get([(x as i32 + x_offset) as f64, (z + z_offset) as f64]) * 0.5 + 0.5).clamp(0.0, 1.0) * 255.0) as u32;
                noise_vec[x].push(noise_val);
                if y_offset <= noise_val {
                    pos_filled += 1
                } else {
                    pos_empty += 1
                }
            }
        }
        println!("pos_filled: {pos_filled}");
        println!("pos_empty: {pos_empty}");
        if pos_empty > 0 && pos_filled == 0 {
            return Self {
                position,
                is_empty: true,
                blocks: [Block::AIR; 5832],
                mesh: Vec::new(),
                vao: 0,
                vbo: 0
            }
        }
        // Check first slice for air, if full of air, chunk empty
        // Check last slice for block, if full of block, chunk full

        for i in 0..ChunkSize::SIZE {
            let [x, y, z] = <ConstShape3u32<18_u32, 18_u32, 18_u32> as ConstShape<3>>::delinearize(i);
            if (x > 0 && x < 17)  && (y > 0 && y < 17) && (z > 0 && z < 17) {
                let noise_val = noise_vec[x as usize - 1][z as usize - 1];
                if y + y_offset <= noise_val {
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
            vbo,
            is_empty: false
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
        println!("quads: {}", buffer.quads.num_quads());
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
        (test_vertices, 0, 0)
        //unsafe {
        //    Chunk::setup_mesh(test_vertices)
        //}
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
        if self.is_empty {
            return
        }
        gl::BindVertexArray(self.vao);
        gl::BindTexture(gl::TEXTURE_2D, *texture);
        shader.use_program();
        let model = glm::translation(&Vec3::new(((self.position.x * 16) - 1) as f32, ((self.position.y * 16) - 1 /* size of cube gen */) as f32, ((self.position.z * 16) - 1) as f32,));
        shader.set_mat4("model", &model);
        gl::DrawArrays(gl::TRIANGLES, 0, (self.mesh.len() / 3) as i32);
        gl::BindVertexArray(0);
        gl::BindTexture(gl::TEXTURE_2D, 0);

    }
}

//pub fn chunk_pos_to_f32(pos: I32Vec3) -> Vec3 {
//    Vec3::new(pos.x as f32, pos.y as f32, pos.z as f32)
//}

use block_mesh::{MergeVoxel, Voxel, VoxelVisibility};


#[derive(Clone, Copy, Eq, PartialEq)]
pub struct BoolVoxel(pub bool);

pub const EMPTY: BoolVoxel = BoolVoxel(false);
pub const FULL: BoolVoxel = BoolVoxel(true);

impl Voxel for BoolVoxel {
    fn get_visibility(&self) -> VoxelVisibility {
        if *self == EMPTY {
            VoxelVisibility::Empty
        } else {
            VoxelVisibility::Opaque
        }
    }
}

impl MergeVoxel for BoolVoxel {
    type MergeValue = Self;

    fn merge_value(&self) -> Self::MergeValue {
        *self
    }

}

#[repr(C)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Block(pub u8);

impl Default for Block {
    fn default() -> Self {
        Self::AIR
    }
}

impl Voxel for Block {
    fn get_visibility(&self) -> VoxelVisibility {
        if *self == Block::AIR {
            VoxelVisibility::Empty
        } else {
            VoxelVisibility::Opaque
        }
    }
}

impl MergeVoxel for Block {
    type MergeValue = Self;

    fn merge_value(&self) -> Self::MergeValue {
        *self
    }
}

impl Block {
    pub const STONE: Block = Block(1);
    pub const AIR: Block = Block(0);
}