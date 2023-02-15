// 2d heightmap for height 
use block_mesh::ndshape::{ConstShape3u32, RuntimeShape, Shape};
use crate::chunk::Chunk;
use noise::{Perlin, Fbm, Seedable};
use crate::glm::vec3;

#[derive(Debug, Default)]
pub struct World {
    //loaded_chunks: Vec<Chunk>,
    //chunks_to_load: Vec<Chunk>,
    chunks: Vec<Chunk>,
    noise: Fbm<Perlin>,
    min: (i32, i32, i32)
}

impl World {
    pub fn new(seed: u32, cube_size: u32) -> Self {
        let shape = RuntimeShape::<u32, 3>::new([cube_size, cube_size, cube_size]);
        let min_val = -((cube_size % 2) as i32);
        let min = (min_val, min_val, min_val);
        let noise: Fbm<Perlin> = Fbm::<Perlin>::default().set_seed(seed);
        let mut chunks = Vec::new();
        for i in 0..shape.size() {
        }
        Self {
            noise,
            chunks,
            min
        }
    }

}