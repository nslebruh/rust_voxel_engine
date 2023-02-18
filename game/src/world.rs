use std::collections::HashSet;

// 2d heightmap for height
use block_mesh::ndshape::{RuntimeShape, Shape};
use engine::glm::IVec3;
use crate::chunk::Chunk;
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
    min: (i32, i32, i32),
    total_size: (u32, u32, u32)
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
            min,
            total_size: (cube_size, cube_size, cube_size)
        };
        output.calculate_visibility([cube_size; 3]);
        for j in 0..output.chunks.len() {
            output.chunks[j].create_mesh();
        }
        output
    }

    pub fn calculate_visibility(&mut self, total_size: [u32; 3]) {
        fn linearise3(p: [u32; 3], total_size: [u32; 3]) -> u32 {
            p[0] + (total_size[0].wrapping_mul(p[1])) + ((total_size[0] * total_size[1]).wrapping_mul(p[2]))
        }
        let colliding_positions: Vec<(IVec3, ChunkDirection)> = vec![
            (IVec3::new( 1,  0,  0), ChunkDirection::PositiveX),
            (IVec3::new(-1,  0,  0), ChunkDirection::NegativeX),
            (IVec3::new( 0,  1,  0), ChunkDirection::PositiveY),
            (IVec3::new( 0, -1,  0), ChunkDirection::NegativeY),
            (IVec3::new( 0,  0,  1), ChunkDirection::PositiveZ),
            (IVec3::new( 0,  0, -1), ChunkDirection::NegativeZ),
        ];

        let mut checked_chunk_directions = vec![[false; 6]; self.chunks.len()];

        // for every chunk in the chunk vec
        for index in 0..self.chunks.len() {
            // and for every direction in the 6 ordinal directions
            for (offset, direction) in colliding_positions.iter() {
                // if the bordering chunk exists
                if let Some(bordering_chunk_position) = self.chunk_positions.get(&(offset + &self.chunks[index].position)) {
                    // get the bordering chunk position
                    let bordering_chunk_index = (linearise3([(bordering_chunk_position.x - self.min.0) as u32, (bordering_chunk_position.y - self.min.1) as u32, (bordering_chunk_position.z - self.min.2) as u32], total_size) - 1) as usize;
                    println!("{bordering_chunk_index}");
                    // match the direction of the bordering chunk
                    match direction {
                        ChunkDirection::PositiveX if !checked_chunk_directions[index][1] => {
                            for y in 1_u32..17 {
                                for z in 1_u32..17 {
                                    // set the right border block to the leftmost block of the bordering chunk
                                    self.chunks[index].blocks[linearise3([17, y, z], total_size) as usize] = self.chunks[bordering_chunk_index].blocks[linearise3([1, y, z], total_size) as usize];
                                    // set the left border block of the bordering chunk to the rightmost block
                                    self.chunks[bordering_chunk_index].blocks[linearise3([0, y, z], total_size) as usize] = self.chunks[index].blocks[linearise3([16, y, z], total_size) as usize];
                                    // set both checked values to true
                                    checked_chunk_directions[index][0] = true;
                                    checked_chunk_directions[bordering_chunk_index][1] = true;
                                }
                            }
                        },
                        ChunkDirection::PositiveY if !checked_chunk_directions[index][3] => {
                            for x in 1..17 {
                                for z in 1..17 {
                                    self.chunks[index].blocks[linearise3([x, 17, z], total_size) as usize] = self.chunks[bordering_chunk_index].blocks[linearise3([x, 1, z], total_size) as usize];
                                    self.chunks[bordering_chunk_index].blocks[linearise3([x, 0, z], total_size) as usize] = self.chunks[index].blocks[linearise3([x, 16, z], total_size) as usize];
                                    checked_chunk_directions[index][2] = true;
                                    checked_chunk_directions[bordering_chunk_index][3] = true;
                                }
                            }
                        },
                        ChunkDirection::PositiveZ if !checked_chunk_directions[index][5] => {
                            for x in 1..17 {
                                for y in 1..17 {
                                    self.chunks[index].blocks[linearise3([x, y, 17], total_size) as usize] = self.chunks[bordering_chunk_index].blocks[linearise3([x, y, 1], total_size) as usize];
                                    self.chunks[bordering_chunk_index].blocks[linearise3([x, y, 0], total_size) as usize] = self.chunks[index].blocks[linearise3([x, y, 16], total_size) as usize];
                                    checked_chunk_directions[index][4] = true;
                                    checked_chunk_directions[bordering_chunk_index][5] = true;
                                }
                            }
                        },
                        ChunkDirection::NegativeX if !checked_chunk_directions[index][0] => {
                            for y in 1..17 {
                                for z in 1..17 {
                                    self.chunks[index].blocks[linearise3([0, y, z], total_size) as usize] = self.chunks[bordering_chunk_index].blocks[linearise3([16, y, z], total_size) as usize];
                                    self.chunks[bordering_chunk_index].blocks[linearise3([17, y, z], total_size) as usize] = self.chunks[index].blocks[linearise3([1, y, z], total_size) as usize];
                                    checked_chunk_directions[index][1] = true;
                                    checked_chunk_directions[bordering_chunk_index][0] = true;
                                }
                            }
                        },
                        ChunkDirection::NegativeY if !checked_chunk_directions[index][2] => {
                            for x in 1..17 {
                                for z in 1..17 {
                                    self.chunks[index].blocks[linearise3([x, 0, z], total_size) as usize] = self.chunks[bordering_chunk_index].blocks[linearise3([x, 16, z], total_size) as usize];
                                    self.chunks[bordering_chunk_index].blocks[linearise3([x, 17, z], total_size) as usize] = self.chunks[index].blocks[linearise3([x, 1, z], total_size) as usize];
                                    checked_chunk_directions[index][3] = true;
                                    checked_chunk_directions[bordering_chunk_index][2] = true;
                                }
                            }
                        },
                        ChunkDirection::NegativeZ if !checked_chunk_directions[index][4] => {
                            for x in 1..17 {
                                for y in 1..17 {
                                    self.chunks[index].blocks[linearise3([x, y, 0], total_size) as usize] = self.chunks[bordering_chunk_index].blocks[linearise3([x, y, 16], total_size) as usize];
                                    self.chunks[bordering_chunk_index].blocks[linearise3([x, y, 17], total_size) as usize] = self.chunks[index].blocks[linearise3([x, y, 1], total_size) as usize];
                                    checked_chunk_directions[index][5] = true;
                                    checked_chunk_directions[bordering_chunk_index][4] = true;
                                }
                            }
                        },
                        _ => {}
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

//fn linearise3(p: [u32; 3]) -> usize {
//    (p[0] + p[0].wrapping_mul(p[1]) + (p[0] * p[1]).wrapping_mul(p[2])) as usize
//}



//fn delinearise3(mut i: u32, total_size: [u32; 3]) -> [u32; 3] {
//    let z = i / (total_size[0] * total_size[1]);
//    i -= z * (total_size[0] * total_size[1]);
//    let y = i / total_size[0];
//    let x = i % total_size[0];
//    [x, y, z]
//}

//fn linearise2(arr: [u32; 2]) -> usize {
//    (arr[0] + arr[0].wrapping_mul(arr[1])) as usize
//}