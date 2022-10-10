use super::World;

pub mod render_system;
pub mod input_system;
pub mod obstacle_system;

pub struct Systems {
    pub list: Vec<Box<dyn EcsSystem>>,
}

impl Systems {
    pub fn new() -> Self {
        Systems { 
            list: vec![],
        }
    }
    pub fn add_system(&mut self, system: Box<dyn EcsSystem>) {
        self.list.push(system);
    }
    pub fn get_system<System: EcsSystem>() {
        
    }
}

pub trait EcsSystem {
    fn update(&mut self, instance: &World);
}