use std::convert::From;

pub struct Vector2f {
    pub x: f32, 
    pub y: f32
}

impl From<Vector2i> for Vector2f {
    fn from(vec: Vector2i) -> Self {
        Self {
            x: vec.x as f32,
            y: vec.y as f32
        }
    }   
}

impl From<Vector2u> for Vector2f {
    fn from(vec: Vector2u) -> Self {
        Self {
            x: vec.x as f32,
            y: vec.y as f32
        }
    }
} 
pub struct Vector2u {
    pub x: u32,
    pub y: u32
}

pub struct Vector2i {
    pub x: i32,
    pub y: i32

}

pub struct Vector3f {

}

pub struct Vector3u {

}
pub struct Vector3i {

}

pub struct Vector4f {

}

pub struct Vector4u {

}

pub struct Vector4i {

}