use cgmath::{InnerSpace, Vector3};
use winit::event::VirtualKeyCode;

use crate::ecs::{
    components::{CameraComponent, CameraControllerComponent, CameraUniformComponent},
    World,
};

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

pub fn update_camera(world: &World) {
    if let (Some(mut camera_vec), Some(mut controller_vec)) = (
        world.borrow_component_vec_mut::<CameraComponent>(),
        world.borrow_component_vec_mut::<CameraControllerComponent>(),
    ) {
        for (camera, controller) in camera_vec.iter_mut().zip(controller_vec.iter_mut()) {
            if let (Some(camera), Some(controller)) = (camera, controller) {
                let x = controller.yaw.to_radians().cos() * controller.pitch.to_radians().cos();
                let y = controller.pitch.to_radians().sin();
                let z = controller.yaw.to_radians().sin() * controller.pitch.to_radians().cos();
                let forward = Vector3::new(x as f32, y as f32, z as f32);
                let forward_norm = forward.normalize();

                if controller.is_forward_pressed {
                    camera.eye += controller.speed * forward_norm;
                }
                if controller.is_backward_pressed {
                    camera.eye -= controller.speed * forward_norm;
                }
                if controller.is_right_pressed {
                    camera.eye += controller.speed * forward_norm.cross(camera.up);
                }
                if controller.is_left_pressed {
                    camera.eye -= controller.speed * forward_norm.cross(camera.up);
                }
                camera.target = camera.eye + forward;
            }
        }
    }
}

pub fn process_input(world: &World) {
    if let Some(mut controller_vec) = world.borrow_component_vec_mut::<CameraControllerComponent>()
    {
        for controller in controller_vec.iter_mut() {
            if let Some(controller) = controller {
                // Keys
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

                // Mouse
                let position = world.resources.input.cursor.position;
                let xoffset = (controller.last_mouse_position_x - position.x) * controller.sensitivity as f64;
                let yoffset = (position.y - controller.last_mouse_position_y) * controller.sensitivity as f64;
                controller.pitch -= yoffset;
                controller.yaw -= xoffset;
                
                if controller.pitch > 89.0 {
                    controller.pitch = 89.0
                }
                if controller.pitch < -89.0 {
                    controller.pitch = -89.0
                }
            }
        }
    }
}

// Camera Uniform
pub fn update_view_projection_matrix(world: &World, aspect: f32) {
    if let (Some(camera_vec), Some(mut camera_uniform_vec)) = (
        world.borrow_component_vec::<CameraComponent>(),
        world.borrow_component_vec_mut::<CameraUniformComponent>(),
    ) {
        for (camera, uniform) in camera_vec.iter().zip(camera_uniform_vec.iter_mut()) {
            if let (Some(camera), Some(uniform)) = (camera, uniform) {
                // 1.
                let view = cgmath::Matrix4::look_at_rh(camera.eye, camera.target, camera.up);
                // 2.
                let proj = cgmath::perspective(
                    cgmath::Deg(camera.fovy),
                    // TODO Store aspect on camera?
                    // camera.aspect,
                    aspect,
                    camera.znear,
                    camera.zfar,
                );

                // 3.
                uniform.view_proj = (OPENGL_TO_WGPU_MATRIX * proj * view).into();
            }
        }
    }
}
