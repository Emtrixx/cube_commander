use rapier3d::prelude::*;

use winit::event::VirtualKeyCode;

use crate::ecs::{
    components::{PlayerComponent, TransformComponent, ControllerComponent},
    World,
};


pub fn update_player(world: &World, y_positon: f32) {
    let mut rng = rand::thread_rng();
    if let Some(mut transform_components) = world.borrow_component_vec_mut::<TransformComponent>() {
        if let Some(mut player_components) = world.borrow_component_vec_mut::<PlayerComponent>() {
            if let Some(mut control_components) = world.borrow_component_vec_mut::<ControllerComponent>()
            
            {
            
            let zip = transform_components
                .iter_mut()
                .zip(control_components.iter_mut());

            for (_, (transform, controller)) in zip
                .filter_map(|(transform, controller)| Some((transform.as_mut()?, controller.as_mut()?))).enumerate()
            {
                // Update position
                // if controller.is_forward_pressed {
                //     transform.position.x  += controller.speed ;
                // }
                // if controller.is_backward_pressed {
                //     transform.position.x -= controller.speed;
                // }
                transform.position.y = y_positon;
                if controller.is_right_pressed {
                    transform.position.x += controller.speed;
                    

                }
                if controller.is_left_pressed {
                    transform.position.x -= controller.speed;
                }

                
             
            }
        }
    }
}
}
pub fn process_input(world: &World) {
    if let Some(mut controller_vec) = world.borrow_component_vec_mut::<ControllerComponent>(){
        if let Some(mut player_components) = world.borrow_component_vec_mut::<PlayerComponent>() 
    
    {
        let zip = controller_vec
                .iter_mut()
                .zip(player_components.iter_mut());
           
                for (_, (controller, Player)) in zip
                .filter_map(|(controller, player)| Some((controller.as_mut()?, player.as_mut()?))).enumerate()
                 {
                   // Keys
                // Das wird vom Controll component geregelt
                controller.is_forward_pressed =
                    *world.resources.input.keys.get(&VirtualKeyCode::W).unwrap_or(&false)
                        || *world.resources.input.keys.get(&VirtualKeyCode::Up).unwrap_or(&false);
                controller.is_left_pressed =
                    *world.resources.input.keys.get(&VirtualKeyCode::A).unwrap_or(&false)
                        || *world.resources.input.keys.get(&VirtualKeyCode::Left).unwrap_or(&false);
                controller.is_backward_pressed =
                    *world.resources.input.keys.get(&VirtualKeyCode::S).unwrap_or(&false)
                        || *world.resources.input.keys.get(&VirtualKeyCode::Down).unwrap_or(&false);
                controller.is_right_pressed =
                    *world.resources.input.keys.get(&VirtualKeyCode::D).unwrap_or(&false)
                        || *world.resources.input.keys.get(&VirtualKeyCode::Right).unwrap_or(&false);

                
            }
        }
    }
}
