// 2d heightmap for height 

use crate::chunk::Chunk;
use noise::{Perlin, Fbm, Seedable};
use crate::glm::vec3;

#[derive(Debug, Default)]
pub struct World {
    //loaded_chunks: Vec<Chunk>,
    //chunks_to_load: Vec<Chunk>,
    chunks: Vec<Chunk>,
    noise: Fbm<Perlin>
}

impl World {
    pub fn new(seed: u32) -> Self {
        let noise: Fbm<Perlin> = Fbm::<Perlin>::default().set_seed(seed);
        let chunks: Vec<Chunk> = vec![
            Chunk::new(vec3(0, 0, 0), &noise)
        ];

        Self {
            noise,
            chunks
        }
    }

}