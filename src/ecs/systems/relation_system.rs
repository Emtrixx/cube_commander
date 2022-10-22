use cgmath::Vector3;

use crate::ecs::{
    components::{ChildComponent, ParentComponent, TransformComponent},
    World,
};

pub fn add_child_component(world: &mut World, parent: usize, child: usize) {
    // TODO only add child and parent components through this function
    // let 
    let mut position = Vector3::new(0.0, 0.0, 0.0);
    if let Some(transform_vec) = world.borrow_component_vec::<TransformComponent>() {
        if let (Some(Some(parent_transform)), Some(Some(child_transform))) = (transform_vec.get(parent), transform_vec.get(child)) {
            position = child_transform.position - parent_transform.position;
        }
    }
    world.add_component_to_entity(parent, 
        ParentComponent::new(child)
    );
    world.add_component_to_entity(child, 
        ChildComponent::new(TransformComponent::from(position))
    );
}

pub fn update_children_transform(world: &World) {
    if let (Some(mut transform_vec), Some(parent_vec), Some(mut child_vec)) = (
        world.borrow_component_vec_mut::<TransformComponent>(),
        world.borrow_component_vec::<ParentComponent>(),
        world.borrow_component_vec_mut::<ChildComponent>(),
    ) {
        let mut parent_info: Vec<(TransformComponent, Vec<usize>)> = vec![];
        let iter = parent_vec.iter().zip(transform_vec.iter());
        for (parent_component, parent_transform) in iter {
            if let (Some(parent_component), Some(parent_transform)) =
                (parent_component, parent_transform)
            {
                let parent_transform_clone = parent_transform.clone();
                let mut child_ids: Vec<usize> = vec![];
                for child_index in parent_component.children.iter() {
                    child_ids.push(*child_index);
                }
                parent_info.push((parent_transform_clone, child_ids));
            }
        }
        println!("Parrents:{:?}", parent_vec);
        println!("Childeren{:?}",child_vec);
    

        for parent in parent_info.iter() {
            for child_index in parent.1.iter() {
                if let Some(Some(child_transform)) = transform_vec.get_mut(*child_index) {
                    if let Some(Some(child_component)) = child_vec.get_mut(*child_index) {
                        child_transform.position = parent.0.position + child_component.transform.position;
                    }
                }
            }
        } 
    }
}
