use std::time::Instant;

use crate::{
    ecs::{
        systems::{render_system, Systems, obstacle_system},
        World,
    },
    render::State,
    world::{init_world},
};
use winit::{
    event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

pub mod shapes;

pub struct GameState {
    pub instance: World,
    pub systems: Systems,
}

pub async fn init_game() {
    let mut game_state = GameState {
        instance: World::new(),
        systems: Systems::new(),
    };

    init_world(&mut game_state.instance);

    // game_state.systems.add_system(
    //     RenderSyst
    // );

    game_loop(game_state).await;
}

async fn game_loop(game_state: GameState) {
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).unwrap();
    let mut state = State::new(&window).await;

    let mut last_render_time = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        // Time
        let now = Instant::now();
        let dt = now - last_render_time;
        last_render_time = now;

        // Systems
        // for system in &game_state.systems.list {
        //     system.update(game_state.instance);
        // }
        // spawn_obstacle(&mut game_state.instance, dt);
        render_system::update_state_mesh_buffer(&game_state.instance, &mut state);
        obstacle_system::update_obstacles(&game_state.instance, dt);

        // RENDER AND INPUT
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                if !state.input(event) {
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    virtual_keycode: Some(keycode),
                                    ..
                                },
                            ..
                        } => match keycode {
                            VirtualKeyCode::Escape => {
                                *control_flow = ControlFlow::Exit;
                            }
                            VirtualKeyCode::F => {
                                window.set_cursor_visible(false);
                                window
                                    .set_cursor_grab(true)
                                    .expect("could not grab mouse cursor");
                            }
                            _ => {}
                        },
                        WindowEvent::Resized(physical_size) => {
                            state.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            state.resize(**new_inner_size);
                        }
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                window.request_redraw();
            }
            _ => {}
        }
        // TODO find right place
        window
            .set_cursor_position(winit::dpi::LogicalPosition::new(50., 50.))
            .unwrap();
    });
}
