use std::time::Duration;

use crate::{
    ecs::{components::{RenderComponent, TransformComponent, ObstacleComponent}, World},
    game::shapes::Shape,
};

pub fn init_world(world: &mut World) {
    
    // Ground
    let ground = world.new_entity();
    world.add_component_to_entity(
        ground,
        RenderComponent {
            mesh: Shape::new_plane([0.0, 1.0, 0.0], 50.0, [0.2, 0.4, 0.2]),
        },
    );

    // Player
    let cube = world.new_entity();
    world.add_component_to_entity(
        cube, 
        RenderComponent {
         mesh: Shape::new_default_cube("cube".to_string(), [0.0, 1.0, 0.0])
        }
    );
    world.add_component_to_entity(
        cube, 
        TransformComponent {
            position: [0.0, 0.1, 0.0],
            rotation: [0.0, 0.0, 0.0],
            scale: [1.0, 1.0, 1.0],
        }
    );

    //  Obstacles
    let mut obstacles: Vec<usize> = vec![];
    for i in 0..5 {
        obstacles.push(world.new_entity());
        world.add_component_to_entity(
            obstacles[i], 
            ObstacleComponent{
                time_alive: Duration::ZERO
            }
        );
        world.add_component_to_entity(
            obstacles[i], 
            RenderComponent {
             mesh: Shape::new_default_cube("cube".to_string(), [0.0, 1.0, 0.0])
            },
        );
        world.add_component_to_entity(
            obstacles[i], 
            TransformComponent {
                position: [0.0, 0.0, -3. * (i + 1) as f32],
                rotation: [0.0, 0.0, 0.0],
                scale: [1.0, 1.0, 1.0],
            }
        );
    }
}