use std::time::Duration;

use cgmath::SquareMatrix;

use crate::game::shapes::Shape;

pub struct RenderComponent {
    pub mesh: Shape,
}

#[derive(Copy, Clone)]
pub struct TransformComponent {
    pub position: [f32; 3],
    pub rotation: [f32; 3],
    pub scale: [f32; 3],
}

// impl TransformComponent {
//     pub fn set_position(&mut self, position: [f32; 3]) {
//         self.position = position;
//     }
//     pub fn get_position(&self) -> [f32; 3] {
//         return self.position;
//     }
//     pub fn set_rotation(&mut self, rotation: [f32; 3]) {
//         self.rotation = rotation;
//     }
//     pub fn get_rotation(&self) -> [f32; 3] {
//         return self.rotation;
//     }
//     pub fn set_scale(&mut self, scale: [f32; 3]) {
//         self.scale = scale;
//     }
//     pub fn get_scale(&self) -> [f32; 3] {
//         return self.scale;
//     }
// }

pub struct ObstacleComponent {
    pub time_alive: Duration,
}

// Camera
pub struct CameraComponent {
    pub eye: cgmath::Point3<f32>,
    pub target: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

#[derive(Debug)]
pub struct CameraControllerComponent {
    pub speed: f32,
    pub sensitivity: f32,
    pub is_forward_pressed: bool,
    pub is_backward_pressed: bool,
    pub is_left_pressed: bool,
    pub is_right_pressed: bool,
    pub last_mouse_position_x: f64,
    pub last_mouse_position_y: f64,
    pub pitch: f64,
    pub yaw: f64,
}

impl CameraControllerComponent {
    pub fn new(speed: f32, sensitivity: f32) -> Self {
        Self {
            speed,
            sensitivity,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
            last_mouse_position_x: 50.,
            last_mouse_position_y: 50.,
            pitch: 0.,
            yaw: -90.,
        }
    }
}

// We need this for Rust to store our data correctly for the shaders
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniformComponent {
    pub view_proj: [[f32; 4]; 4],
}

impl CameraUniformComponent {
    pub fn new() -> Self {
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }
}


// Parent Child Relation
pub struct ParentComponent {
    pub children: Vec<usize>,
}

pub struct ChildComponent {
    pub transform: TransformComponent
}

impl ChildComponent {
    pub fn set_transform(&mut self, position: [f32;3]) {
        self.transform.position = position;
    }
    pub fn get_transform(&self) -> [f32;3] {
        self.transform.position
    }
}