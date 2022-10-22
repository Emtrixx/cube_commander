use std::{cell::{RefCell,RefMut, Ref}, borrow::Borrow};



use self::systems::input_system::Input;

pub mod systems;
pub mod components;

trait ComponentVec {
    fn push_none(&mut self);
    fn remove(&mut self, index: usize);
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

impl<T: 'static> ComponentVec for RefCell<Vec<Option<T>>> {
    fn push_none(&mut self) {
        self.get_mut().push(None);
    }
    fn remove(&mut self, index: usize) {
        self.get_mut().remove(index).unwrap();
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }
}

pub struct Resources {
    input: Input,
}

impl Resources {
    pub fn new() -> Self {
        Self {
            input: Input::new(),
        }
    }
}

pub struct World {
    entities_count: usize,
    component_vecs: Vec<Box<dyn ComponentVec>>,
    resources: Resources,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities_count: 0,
            component_vecs: Vec::new(),
            resources: Resources::new(),
        }
    }

    pub fn new_entity(&mut self) -> usize {
        let entity_id = self.entities_count;
        for component_vec in self.component_vecs.iter_mut() {
            component_vec.push_none();
        }
        self.entities_count += 1;
        entity_id
    }

    pub fn remove_entity(&mut self, entity: usize) {
        for component_vec in self.component_vecs.iter_mut() {
            component_vec.as_mut().remove(entity);
        }
    }

    pub fn add_component_to_entity<ComponentType: 'static>(
        &mut self,
        entity: usize,
        component: ComponentType,
    ) {
        // Find matching component vec and insert component
        for component_vec in self.component_vecs.iter_mut() {
            if let Some(component_vec) = component_vec
                .as_any_mut()
                .downcast_mut::<RefCell<Vec<Option<ComponentType>>>>()
            {
                component_vec.get_mut()[entity] = Some(component);
                return;
            }
        }

        // No matching component vec found, create new one and fill all entities with none and insert new component
        let mut new_component_vec: Vec<Option<ComponentType>> = Vec::with_capacity(self.entities_count);
        for _ in 0..self.entities_count {
            new_component_vec.push(None);
        }
        new_component_vec[entity] = Some(component);

        self.component_vecs.push(Box::new(RefCell::new(new_component_vec)));
    }

    pub fn remove_component_from_entity<ComponentType: 'static>(
        &mut self, 
        entity: usize,
        _: ComponentType,
    ) {
        // Find matching component vec and insert component
        for component_vec in self.component_vecs.iter_mut() {
            if let Some(component_vec) = component_vec
                .as_any_mut()
                .downcast_mut::<RefCell<Vec<Option<ComponentType>>>>()
            {
                component_vec.get_mut()[entity] = None;
                return;
            }
        }
    }

    pub fn borrow_component_vec<ComponentType: 'static>(&self) -> Option<Ref<Vec<Option<ComponentType>>>>  {
        for component_vec in self.component_vecs.iter() {
            if let Some(component_vec) = component_vec
                .as_any()
                .downcast_ref::<RefCell<Vec<Option<ComponentType>>>>()
            {
                return Some(component_vec.borrow());
            }
        }
        None
    }

    pub fn borrow_component_vec_mut<ComponentType: 'static>(&self) -> Option<RefMut<Vec<Option<ComponentType>>>>  {
        for component_vec in self.component_vecs.iter() {
            if let Some(component_vec) = component_vec
                .as_any()
                .downcast_ref::<RefCell<Vec<Option<ComponentType>>>>()
            {
                return Some(component_vec.borrow_mut());
            }
        }
        None
    }

    // pub fn borrow_component<ComponentType: 'static>(&self, entity: usize) -> Option<RefMut<Option<ComponentType>>> {
    //     for component_vec in self.component_vecs.iter_mut() {
    //         if let Some(component_vec) = component_vec
    //             .as_any_mut()
    //             .downcast_mut::<RefCell<Vec<Option<ComponentType>>>>()
    //         {

    //             return component_vec.borrow_mut();

    //             // if let Some(component) = &component_vec.get_mut()[entity] {}
    //             // return component_vec.borrow_mut().get_mut(entity)
    //             // if let Some(component) = component_vec.borrow().get_mut(entity) {
    //             //     return component.as_mut();
    //             // }
    //         }
    //     }
    //     None
    // }
}


// Resources
// struct Res<T> (T);

// impl<T> Res<T> 
// where T: 'static {
//     pub fn new(content: T) -> Self {
//         Res(content)
//     }
// }