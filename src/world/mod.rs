use std::time::Duration;
use cgmath::Vector3;
use crate::ecs::components::ControllerComponent;
use crate::utils::colors::{RED,BLUE};
use crate::ecs::systems::relation_system;


use crate::{
    ecs::{components::{RenderComponent, TransformComponent, ObstacleComponent, CameraComponent, CameraUniformComponent, CameraControllerComponent, PlayerComponent}, World},
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
    let player = world.new_entity();
    world.add_component_to_entity(
        player, 
        TransformComponent::new()
    );
    world.add_component_to_entity(
        player, 
        RenderComponent {
         mesh: Shape::new_default_cube(RED)
        }
    );
    world.add_component_to_entity(
        player,
        PlayerComponent{});
    world.add_component_to_entity(
        player,
        ControllerComponent::new(0.05, 0.1)
    );

    // Camera
    let camera_position = (0.0, 2.0, 4.0);
    let camera = world.new_entity();
    world.add_component_to_entity(camera, 
        CameraComponent {
            // position the camera one unit up and 2 units back
            // +z is out of the screen
            eye: camera_position.into(),
            // have it look at the origin
            target: (0.0, 0.0, 0.0).into(),
            // which way is "up"
            up: cgmath::Vector3::unit_y(),
            // TODO fet real aspect
            aspect: 16 as f32 / 9 as f32,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        }
    );
    world.add_component_to_entity(camera,
        CameraUniformComponent::new()
    );
    world.add_component_to_entity(camera, 
        CameraControllerComponent::new(0.05, 0.1)
    );
    world.add_component_to_entity(camera, 
        TransformComponent::from(Vector3::new(camera_position.0, camera_position.1, camera_position.2))
    );
    relation_system::add_child_component(world, player, camera);
    

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
             mesh: Shape::new_default_cube(BLUE)
            },
        );
        world.add_component_to_entity(
            obstacles[i], 
            TransformComponent::from(Vector3::new(0.0, 0.0, -10. + (-5. * (i + 1) as f32)))
        );
    }
}
