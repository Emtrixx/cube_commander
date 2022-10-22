use std::time::Duration;
use rand::Rng;

use crate::ecs::{
    components::{PlayerComponent, TransformComponent},
    World,
};

pub fn update_player(world: &World) {
    let mut rng = rand::thread_rng();
    if let Some(mut transform_components) = world.borrow_component_vec_mut::<TransformComponent>() {
        if let Some(mut obstacle_components) = world.borrow_component_vec_mut::<PlayerComponent>()
        {
            let zip = transform_components
                .iter_mut()
                .zip(obstacle_components.iter_mut());

            for (_, (transform, player)) in zip
                .filter_map(|(transform, player)| Some((transform.as_mut()?, player.as_mut()?))).enumerate()
            {
                // Update position
                

                // TODO Use Time
             
            }
        }
    }
}
