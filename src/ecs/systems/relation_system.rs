use crate::ecs::{
    components::{ChildComponent, ParentComponent, TransformComponent},
    World,
};

pub fn add_child_component(_parent: usize, _child: usize) {
    // TODO only add child and parent components through this function
    // Or make entity as struct and add function there
}

pub fn update_children_transform(world: &World) {
    if let (Some(transform_vec), Some(parent_vec)) = (
        world.borrow_component_vec_mut::<TransformComponent>(),
        world.borrow_component_vec::<ParentComponent>(),
    ) {
        for (parent_component, parent_transform) in parent_vec.iter().zip(transform_vec.iter()) {
            if let (Some(parent_component), Some(parent_transform)) =
            (parent_component, parent_transform)
            {
                for child in parent_component.children.iter() {
                    if let (Some(mut child_transform), Some(child_component)) = (
                        world.borrow_component::<TransformComponent>(*child),
                        world.borrow_component::<ChildComponent>(*child),
                    ) {
                        // TODO if parent also has childcomponent use this instead of parent_transform
                        // TODO get this right
                        // child_transform.position[0] = parent_transform.position[0] + child_component.transform.position[0];
                        // child_transform.position[1] = parent_transform.position[1] + child_component.transform.position[1];
                        // child_transform.position[2] = parent_transform.position[2] + child_component.transform.position[2];
                    }
                }
            }
        }
    }
}
