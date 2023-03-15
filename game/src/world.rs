use std::collections::HashSet;

// 2d heightmap for height
use block_mesh::ndshape::{RuntimeShape, Shape, ConstShape3u32, ConstShape};
use engine::glm::IVec3;
use crate::{chunk::Chunk, player::Player};
use noise::{Perlin, Fbm, Seedable, MultiFractal};
use crate::glm::vec3;

#[derive(Debug, Default)]
pub struct World {
    pub current_chunk: IVec3,
    //loaded_chunks: Vec<Chunk>,
    //chunks_to_load: Vec<Chunk>,
    //chunks_to_unload: Vec<Chunk>,
    pub chunk_positions: HashSet<IVec3>,
    pub chunks: Vec<Chunk>,
    pub player: Player,
    noise: Fbm<Perlin>,
    pub min: (i32, i32, i32),
    total_size: (u32, u32, u32)
}

impl World {
    pub fn new(seed: u32, cube_size: u32, tall: bool, player: Player) -> Self {
        let min_val = -((cube_size % 2) as i32);
        let shape;
        let min;
        let total_size;
        if tall {
            shape = RuntimeShape::<u32, 3>::new([cube_size, 16, cube_size]);
            min = (min_val, -16, min_val);
            total_size = (cube_size, 16, cube_size);
        } else {
            shape = RuntimeShape::<u32, 3>::new([cube_size; 3]);
            min = (min_val, min_val, min_val);
            total_size = (cube_size, cube_size, cube_size);
        }

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
            current_chunk: IVec3::new(0, 0, 0),
            noise,
            chunks,
            chunk_positions,
            min,
            total_size,
            player
        };
        for k in 0..shape.usize() {
            output.calculate_visibility(output.chunks[k].position);
        }
        for j in 0..shape.usize() {
            output.chunks[j].create_mesh();
        }
        output
    }

    fn calculate_visibility(&mut self, chunk_position: IVec3) {
        let normal_chunk_pos = [(chunk_position.x - self.min.0) as u32, chunk_position.y as u32, (chunk_position.z - self.min.2) as u32];
        let runtime_shape = RuntimeShape::<u32, 3>::new([self.total_size.0, self.total_size.1, self.total_size.2]);
        //println!("normal_chunk_pos: {:?}", normal_chunk_pos);
        let chunk_index = runtime_shape.linearize(normal_chunk_pos) as usize;
        let mut new_chunk = self.chunks[chunk_index].clone();

        if self.chunk_positions.contains(&(chunk_position + IVec3::new(1, 0, 0))) {
            let pos = chunk_position + IVec3::new(1, 0, 0);
            let normal_pos = [(pos.x - self.min.0) as u32, pos.y as u32, (pos.z - self.min.2) as u32];
            let border_chunk_index = runtime_shape.linearize(normal_pos) as usize;
            let bordering_chunk = self.chunks[border_chunk_index].clone();
            //println!("Chunk pos: {}, {}, {}, x+ pos: {}, {}, {}", chunk_position.x, chunk_position.y, chunk_position.z, pos.x, pos.y, pos.z);
            for y in 1_u32..17 {
                for z in 1_u32..17 {
                    let chunk_block_index = <ConstShape3u32<18_u32, 18_u32, 18_u32> as ConstShape<3>>::linearize([17, y, z]) as usize;
                    let bordering_block_index = <ConstShape3u32<18_u32, 18_u32, 18_u32> as ConstShape<3>>::linearize([1, y, z]) as usize;
                    let bordering_block = bordering_chunk.blocks[bordering_block_index];
                    new_chunk.blocks[chunk_block_index] = bordering_block;
                }
            }
        }

        if self.chunk_positions.contains(&(chunk_position + IVec3::new(-1, 0, 0))) {
            let pos = chunk_position + IVec3::new(-1, 0, 0);
            let normal_pos = [(pos.x - self.min.0) as u32, pos.y as u32, (pos.z - self.min.2) as u32];
            let border_chunk_index = runtime_shape.linearize(normal_pos) as usize;
            let bordering_chunk = self.chunks[border_chunk_index].clone();
            //println!("Chunk pos: {}, {}, {},  x- pos: {}, {}, {}", chunk_position.x, chunk_position.y, chunk_position.z, pos.x, pos.y, pos.z);
            for y in 1_u32..17 {
                for z in 1_u32..17 {
                    let chunk_block_index = <ConstShape3u32<18_u32, 18_u32, 18_u32> as ConstShape<3>>::linearize([0, y, z]) as usize;
                    let bordering_block_index = <ConstShape3u32<18_u32, 18_u32, 18_u32> as ConstShape<3>>::linearize([16, y, z]) as usize;
                    let bordering_block = bordering_chunk.blocks[bordering_block_index];
                    new_chunk.blocks[chunk_block_index] = bordering_block;
                }
            }
        }

        if self.chunk_positions.contains(&(chunk_position + IVec3::new(0, 1, 0))) {
            let pos = chunk_position + IVec3::new(0, 1, 0);
            let normal_pos = [(pos.x - self.min.0) as u32, pos.y as u32, (pos.z - self.min.2) as u32];
            let border_chunk_index = runtime_shape.linearize(normal_pos) as usize;
            let bordering_chunk = self.chunks[border_chunk_index].clone();
            //println!("Chunk pos: {}, {}, {}, y+ pos: {}, {}, {}", chunk_position.x, chunk_position.y, chunk_position.z, pos.x, pos.y, pos.z);
            for x in 1_u32..17 {
                for z in 1_u32..17 {
                    let chunk_block_index = <ConstShape3u32<18_u32, 18_u32, 18_u32> as ConstShape<3>>::linearize([x, 17, z]) as usize;
                    let bordering_block_index = <ConstShape3u32<18_u32, 18_u32, 18_u32> as ConstShape<3>>::linearize([x, 1, z]) as usize;
                    let bordering_block = bordering_chunk.blocks[bordering_block_index];
                    new_chunk.blocks[chunk_block_index] = bordering_block;
                }
            }
        }

        if self.chunk_positions.contains(&(chunk_position + IVec3::new(0, -1, 0))) {
            let pos = chunk_position + IVec3::new(0, -1, 0);
            let normal_pos = [(pos.x - self.min.0) as u32, pos.y as u32, (pos.z - self.min.2) as u32];
            let border_chunk_index = runtime_shape.linearize(normal_pos) as usize;
            let bordering_chunk = self.chunks[border_chunk_index].clone();
            //println!("Chunk pos: {}, {}, {}, y+ pos: {}, {}, {}", chunk_position.x, chunk_position.y, chunk_position.z, pos.x, pos.y, pos.z);
            for x in 1_u32..17 {
                for z in 1_u32..17 {
                    let chunk_block_index = <ConstShape3u32<18_u32, 18_u32, 18_u32> as ConstShape<3>>::linearize([x, 0, z]) as usize;
                    let bordering_block_index = <ConstShape3u32<18_u32, 18_u32, 18_u32> as ConstShape<3>>::linearize([x, 16, z]) as usize;
                    let bordering_block = bordering_chunk.blocks[bordering_block_index];
                    new_chunk.blocks[chunk_block_index] = bordering_block;
                }
            }
        }

        if self.chunk_positions.contains(&(chunk_position + IVec3::new(0, 0, 1))) {
            let pos = chunk_position + IVec3::new(0, 0, 1);
            let normal_pos = [(pos.x - self.min.0) as u32, pos.y as u32, (pos.z - self.min.2) as u32];
            let border_chunk_index = runtime_shape.linearize(normal_pos) as usize;
            let bordering_chunk = self.chunks[border_chunk_index].clone();
            //println!("Chunk pos: {}, {}, {}, z+ pos: {}, {}, {}", chunk_position.x, chunk_position.y, chunk_position.z, pos.x, pos.y, pos.z);
            for x in 1_u32..17 {
                for y in 1_u32..17 {
                    let chunk_block_index = <ConstShape3u32<18_u32, 18_u32, 18_u32> as ConstShape<3>>::linearize([x, y, 17]) as usize;
                    let bordering_block_index = <ConstShape3u32<18_u32, 18_u32, 18_u32> as ConstShape<3>>::linearize([x, y, 1]) as usize;
                    let bordering_block = bordering_chunk.blocks[bordering_block_index];
                    new_chunk.blocks[chunk_block_index] = bordering_block;
                }
            }
        }

        if self.chunk_positions.contains(&(chunk_position + IVec3::new(0, 0, -1))) {
            let pos = chunk_position + IVec3::new(0, 0, -1);
            let normal_pos = [(pos.x - self.min.0) as u32, pos.y as u32, (pos.z - self.min.2) as u32];
            let border_chunk_index = runtime_shape.linearize(normal_pos) as usize;
            let bordering_chunk = self.chunks[border_chunk_index].clone();
            //println!("Chunk pos: {}, {}, {}, z+ pos: {}, {}, {}", chunk_position.x, chunk_position.y, chunk_position.z, pos.x, pos.y, pos.z);
            for x in 1_u32..17 {
                for y in 1_u32..17 {
                    let chunk_block_index = <ConstShape3u32<18_u32, 18_u32, 18_u32> as ConstShape<3>>::linearize([x, y, 0]) as usize;
                    let bordering_block_index = <ConstShape3u32<18_u32, 18_u32, 18_u32> as ConstShape<3>>::linearize([x, y, 16]) as usize;
                    let bordering_block = bordering_chunk.blocks[bordering_block_index];
                    new_chunk.blocks[chunk_block_index] = bordering_block;
                }
            }
        }
        self.chunks[chunk_index] = new_chunk;
    }

    pub fn calc_chunk_index(&self, pos: IVec3) -> usize {
        let shape = RuntimeShape::<u32, 3>::new([self.total_size.0, self.total_size.1, self.total_size.2]);
        let normal_pos = [(pos.x - self.min.0) as u32, pos.y as u32, (pos.z - self.min.2) as u32]; 
        shape.linearize(normal_pos) as usize
        
    }
}
