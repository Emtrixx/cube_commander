use std::time::Duration;

use crate::game::shapes::Shape;

pub struct RenderComponent {
    pub mesh: Shape,
}

pub struct TransformComponent {
    pub position: [f32; 3],
    pub rotation: [f32; 3],
    pub scale: [f32; 3],
}

pub struct ObstacleComponent {
    pub time_alive: Duration,
}