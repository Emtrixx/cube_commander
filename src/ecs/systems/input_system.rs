use winit::{event::{WindowEvent, KeyboardInput, ElementState, VirtualKeyCode, DeviceId}, dpi::PhysicalPosition};
use std::collections::HashMap;
use crate::ecs::World;

// Ressource
pub struct Input {
    pub keys: HashMap<VirtualKeyCode, bool>,
    pub cursor: Cursor, 
}

impl Input {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
            // TODO remove unsafe init dummy data
            cursor: Cursor { position: PhysicalPosition { x: 0.0, y: 0.0 }, device_id: unsafe{DeviceId::dummy()} },
        }
    }
}

pub struct Cursor {
    pub position: PhysicalPosition<f64>,
    pub device_id: DeviceId,
}



// System
pub fn process_input_events(world: &mut World, event: &WindowEvent) -> bool {
    match event {
        WindowEvent::KeyboardInput {
            input: KeyboardInput {
                state,
                virtual_keycode: Some(keycode),
                ..
            },
            ..
        } => {
            let is_pressed = *state == ElementState::Pressed;
            world.resources.input.keys.insert(*keycode, is_pressed);
            // println!("Input: {:?}", world.resources.input);
            true
        },
        WindowEvent::CursorMoved { position, device_id, ..} => {
            world.resources.input.cursor.device_id = *device_id;
            world.resources.input.cursor.position = *position;
            true
        }
        _ => false,
    }
}