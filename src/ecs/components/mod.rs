use std::time::Duration;

use cgmath::{SquareMatrix, Vector3};

use crate::game::shapes::Shape;

pub struct RenderComponent {
    pub mesh: Shape,
}

#[derive(Clone,Debug)]
pub struct TransformComponent {
    pub position: Vector3<f32>,
    pub rotation: Vector3<f32>,
    pub scale: Vector3<f32>,
}

impl TransformComponent {
    pub fn new() -> Self {
        Self {
            position: Vector3::new(0.0, 0.0, 0.0),
            rotation: Vector3::new(0.0, 0.0, 0.0),
            scale: Vector3::new(1.0, 1.0, 1.0),
        }
    }
    pub fn from(position: Vector3<f32>) -> Self {
        Self {
            position,
            rotation: Vector3::new(0.0, 0.0, 0.0),
            scale: Vector3::new(1.0, 1.0, 1.0),
        }
    }
    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.position.x = x;
        self.position.y = y;
        self.position.z = z;
    }
    pub fn get_position(&self) -> Vector3<f32> {
        return self.position;
    }
    pub fn set_rotation(&mut self, rotation: Vector3<f32>) {
        self.rotation = rotation;
    }
    pub fn get_rotation(&self) -> Vector3<f32> {
        return self.rotation;
    }
    pub fn set_scale(&mut self, scale: Vector3<f32>) {
        self.scale = scale;
    }
    pub fn get_scale(&self) -> Vector3<f32> {
        return self.scale;
    }
}

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
            last_mouse_position_x: 75.,
            last_mouse_position_y: 75.,
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
#[derive(Debug)]
pub struct ParentComponent {
    pub children: Vec<usize>,
    
}

impl ParentComponent {
    pub fn new(child: usize) -> Self {
        Self {
            children: vec![child],
        }
    }
}
#[derive(Debug)]
pub struct ChildComponent {
    pub transform: TransformComponent,
}

impl ChildComponent {
    pub fn new(transform: TransformComponent) -> Self {
        Self {
            transform,
        }
    }
    pub fn set_transform(&mut self, position: Vector3<f32>) {
        self.transform.position = position;
    }
    pub fn get_transform(&self) -> Vector3<f32> {
        self.transform.position
    }
}
// Player Component
pub struct PlayerComponent {}

pub struct ControllerComponent {
    pub speed: f32,
    pub sensitivity: f32,
    pub is_forward_pressed: bool,
    pub is_backward_pressed: bool,
    pub is_left_pressed: bool,
    pub is_right_pressed: bool,

    
}
impl ControllerComponent{
    pub fn new(speed:f32,sensitivity:f32 ) -> Self {
        Self {
             speed: speed,
             sensitivity: sensitivity,
             is_forward_pressed: false,
             is_backward_pressed: false,
             is_left_pressed: false,
             is_right_pressed: false
        }

    }
}
   

