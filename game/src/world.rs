use std::collections::HashSet;

// 2d heightmap for height
use block_mesh::ndshape::{RuntimeShape, Shape};
use engine::glm::IVec3;
use crate::{chunk::Chunk, block::Block};
use noise::{Perlin, Fbm, Seedable, MultiFractal};
use crate::glm::vec3;

#[derive(Debug, Default)]
pub struct World {
    //loaded_chunks: Vec<Chunk>,
    //chunks_to_load: Vec<Chunk>,
    //chunks_to_unload: Vec<Chunk>,
    chunk_positions: HashSet<IVec3>,
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
        //let mut chunk_positions = Vec::new();
        let mut chunk_positions: HashSet<IVec3> = HashSet::new();

        for i in 0..shape.size() {
            let [x, y, z] = shape.delinearize(i);
            chunks.push(Chunk::new(vec3(x as i32 + min_val, y as i32, z as i32 + min_val), &noise));
            //chunk_positions.push(vec3(x as i32 + min_val, y as i32, z as i32 + min_val))
            chunk_positions.insert(vec3(x as i32 + min_val, y as i32, z as i32 + min_val));
        }
        let mut output = Self {
            noise,
            chunks,
            chunk_positions,
            min
        };
        output.calculate_visibility();
        output
    }

    pub fn calculate_visibility(&mut self) {
        let colliding_positions: Vec<(IVec3, ChunkDirection)> = vec![
            (IVec3::new( 1,  0,  0), ChunkDirection::PositiveX),
            (IVec3::new(-1,  0,  0), ChunkDirection::NegativeX),
            (IVec3::new( 0,  1,  0), ChunkDirection::PositiveY),
            (IVec3::new( 0, -1,  0), ChunkDirection::NegativeY),
            (IVec3::new( 0,  0,  1), ChunkDirection::PositiveZ),
            (IVec3::new( 0,  0, -1), ChunkDirection::NegativeZ),
        ];
        for chunk in self.chunks.clone().iter_mut() {
            for (position, direction) in colliding_positions.iter() {
                if let Some(x) = self.chunk_positions.get(&(position + chunk.position)) {
                    let colliding_chunk_blocks = &self.chunks[linearise3([x.x as u32, x.y as u32, x.z as u32])].blocks.unwrap();
                    match direction {
                        ChunkDirection::PositiveX => {
                            for z in 1_u32..17 {
                                for y in 1_u32..17 {
                                    let i = linearise3([16, y, z]);
                                    if colliding_chunk_blocks[i] == Block::AIR {
                                        break
                                    }
                                }
                            }
                        },
                        ChunkDirection::PositiveY => {

                        },
                        ChunkDirection::PositiveZ => {

                        },
                        ChunkDirection::NegativeX => {

                        },
                        ChunkDirection::NegativeY => {

                        },
                        ChunkDirection::NegativeZ => {

                        },
                    }
                }
            }
        }
    }
}

pub enum ChunkDirection {
    PositiveX,
    PositiveY,
    PositiveZ,
    NegativeX,
    NegativeY,
    NegativeZ
}

//[1, X, X * Y]
// p[0] + Self::STRIDES[1].wrapping_mul(p[1]) + Self::STRIDES[2].wrapping_mul(p[2])

fn linearise3(p: [u32; 3]) -> usize {
    (p[0] + p[0].wrapping_mul(p[1]) + (p[0] * p[1]).wrapping_mul(p[2])) as usize
}

fn delinearise3(mut i: u32, total_size: [u32; 3]) -> [u32; 3] {
    let z = i / (total_size[0] * total_size[1]);
    i -= z * (total_size[0] * total_size[1]);
    let y = i / total_size[0];
    let x = i % total_size[0];
    [x, y, z]
}

fn linearise2(arr: [u32; 2]) -> usize {
    (arr[0] + arr[0].wrapping_mul(arr[1])) as usize
}