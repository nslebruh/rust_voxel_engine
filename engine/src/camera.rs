use nalgebra::*;

type Vector3 = nalgebra::Vector3<f32>;
type Point3 = nalgebra::Point3<f32>;


#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum CameraMovement {
    Forward,
    Backward,
    Left,
    Right,
    Up,
    Down
}

const YAW: f32 = -90.0;
const PITCH: f32 = 0.0;
const SPEED: f32 = 2.5;
const SENSITIVTY: f32 = 0.1;
const ZOOM: f32 = 45.0;

/// Camera struct
/// 
/// Implements Debug
/// 
/// 
#[derive(Debug)]
pub struct Camera {
    /// 
    pub position: Point3,
    pub front: Vector3,
    pub up: Vector3,
    pub right: Vector3,
    pub world_up: Vector3,
    // Euler Angles
    pub yaw: f32,
    pub pitch: f32,
    // Camera options
    pub movement_speed: f32,
    pub mouse_sensitivity: f32,
    pub zoom: f32,
    pub cursor_mode: bool
}

impl Default for Camera {
    fn default() -> Self {
        let mut camera = Self {
            position: Point3::new(0.0, 0.0, 0.0),
            front: Vector3::new(0.0, 0.0, -1.0),
            up: Vector3::new(0.0, 0.0, 0.0),
            right: Vector3::new(0.0, 0.0, 0.0),
            world_up: Vector3::new(0.0, 1.0, 0.0),
            yaw: YAW,
            pitch: PITCH,
            movement_speed: SPEED,
            mouse_sensitivity: SENSITIVTY,
            zoom: ZOOM,
            cursor_mode: false

        };
        camera.update_camera_vectors();
        camera
    }
}

impl Camera {
    pub fn get_view_matrix(&self) -> Matrix4<f32> {
        Matrix4::<f32>::look_at_rh(&self.position, &(self.position + &self.front), &self.up)
    }

    pub fn process_action_input(&mut self, direction: CameraMovement, delta_time: &f32) {
        let velocity = self.movement_speed * *delta_time;

        match direction {
            CameraMovement::Forward => {
                self.position += self.front * velocity;
            },
            CameraMovement::Backward => {
                self.position += -(self.front * velocity);
            },
            CameraMovement::Left => {
                self.position += -(self.right * velocity);
            },
            CameraMovement::Right => {
                self.position += self.right * velocity;
            },
            CameraMovement::Up => {
                self.position += self.world_up * velocity;
            },
            CameraMovement::Down => {
                self.position += -(self.world_up * velocity);
            },
        }
    }

    pub fn process_mouse_input(&mut self, mut x_offset: f32, mut y_offset: f32, constrain_pitch: bool) {
        x_offset *= self.mouse_sensitivity;
        y_offset *= self.mouse_sensitivity;

        self.yaw += x_offset;
        self.pitch += y_offset;

        if constrain_pitch {
            if self.pitch > 89.0 {
                self.pitch = 89.0
            }
            if self.pitch < -89.0 {
                self.pitch = -89.0
            }
        }

        self.update_camera_vectors()
    }

    pub fn process_scroll_input(&mut self, y_offset: f32) {
        if self.zoom >= 1.0 && self.zoom <= 45.0 {
            self.zoom -= y_offset
        }

        if self.zoom <= 1.0 {
            self.zoom = 1.0;
        }
        if self.zoom >= 45.0 {
            self.zoom = 45.0;
        }
    }

    fn update_camera_vectors(&mut self) {
        // Calculate the new Front vector
        let front = Vector3::new (
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        );
        self.front = front.normalize();
        // Also re-calculate the Right and Up vector
        self.right = self.front.cross(&self.world_up).normalize(); // Normalize the vectors, because their length gets closer to 0 the more you look up or down which results in slower movement.
        self.up = self.right.cross(&self.front).normalize();
    }
}