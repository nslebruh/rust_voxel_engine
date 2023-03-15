use engine::{camera::Camera, glm::IVec3};

#[derive(Debug, Default)]
pub struct Player {
  pub camera: Camera,
  pub current_chunk: IVec3,
  pub floored_normal_position: (u32, u32, u32),
  pub inventory: Inventory,
  pub username: String,
  pub health: u32,
}

impl Player {
  pub fn new(camera: Camera) -> Self {
    Self {
      camera, 
      ..Default::default()
    }
  }

}

#[derive(Debug, Default)]
pub struct Inventory {
  armour_slots: [Option<Item>; 4],
  inventory_slots: [Option<Item>; 27],
  hotbar: [Option<Item>; 9]
}

#[derive(Debug)]
pub enum Item {
  NoItem,
  Dirt,
  Grass,
  Stone,

}