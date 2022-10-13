use crate::{render::{State}, ecs::{World, components::{RenderComponent, TransformComponent, CameraUniformComponent}}, game::shapes::Shape};

pub fn update_state_mesh_buffer(world: &World, state: &mut State) {
    let transform_components = world.borrow_component_vec::<TransformComponent>().unwrap();
    if let Some(mut render_components) = world.borrow_component_vec_mut::<RenderComponent>() {
        let mut meshes: Vec<Shape> = vec![];
        for (index, render_component) in render_components.iter_mut().enumerate() {
            // If there is a render component
            if let Some(render_component) = render_component {
                let mut mesh = render_component.mesh.clone();
                // If there is also a transform component
                if let Some(transform) = transform_components.get(index).unwrap() {
                    for v in 0..render_component.mesh.vertices.len() {
                        for i in 0..3 {
                            mesh.vertices[v].position[i] += transform.position[i];
                        }
                    }
                }

                meshes.push(mesh);
            }
        }
        state.set_mesh_buffer(meshes);
    }
}

pub fn set_camera_uniform(world: &World, state: &mut State) {
    if let Some(uniform_vec) = world.borrow_component_vec::<CameraUniformComponent>()
    {
        for uniform in uniform_vec.iter() {
            if let Some(uniform) = uniform {
                state.set_camera_uniform(*uniform);
            }
        }
    }
}