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
    pub const AIR: Block = Block(0);
    pub const WATER: Block = Block(4);
    pub const DIRT: Block = Block(1);
    pub const GRASS: Block = Block(2);
    pub const STONE: Block = Block(3);
}