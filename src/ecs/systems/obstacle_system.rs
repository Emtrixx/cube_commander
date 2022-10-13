use std::time::Duration;
use rand::Rng;

use crate::ecs::{
    components::{ObstacleComponent, TransformComponent},
    World,
};

pub fn update_obstacles(world: &World, dt: Duration) {
    let mut rng = rand::thread_rng();
    if let Some(mut transform_components) = world.borrow_component_vec_mut::<TransformComponent>() {
        if let Some(mut obstacle_components) = world.borrow_component_vec_mut::<ObstacleComponent>()
        {
            let zip = transform_components
                .iter_mut()
                .zip(obstacle_components.iter_mut());

            for (_, (transform, obstacle)) in zip
                .filter_map(|(transform, obstacle)| Some((transform.as_mut()?, obstacle.as_mut()?))).enumerate()
            {
                // Update position
                transform.position[2] += 2. * dt.as_secs_f32();

                // TODO Use Time
                obstacle.time_alive += dt;
                if obstacle.time_alive.as_secs() > 2 && transform.position[2] > 3.0 {
                    obstacle.time_alive = Duration::new(0, 0);
                    transform.position = [rng.gen_range(0.0..16.0) - 8.0, 0.0, -20.0];
                }
            }
        }
    }
}
