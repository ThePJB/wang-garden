mod renderer;
mod rect;
mod level;
mod levels;
mod kmath;
mod application;

use glow::*;
use std::error::Error;
use glam::{Mat4};
use kmath::*;
use renderer::*;
use rect::*;
use level::*;
use levels::*;
use application::*;
use std::collections::HashSet;
use std::time::{Duration, SystemTime};

fn main() -> Result<(), Box<dyn Error>> {

    let mut window_x = 1600.0;
    let mut window_y = 1200.0;

    let projection_mat = Mat4::orthographic_lh(0.0, 1.0, 1.0, 0.0, 1000.0, 0.0);
    let projection_inverse = projection_mat.inverse();


    unsafe {
        let event_loop = glutin::event_loop::EventLoop::new();
        let window_builder = glutin::window::WindowBuilder::new()
            .with_title("Wang's Garden")
            //.with_inner_size(glutin::dpi::LogicalSize::new(window_x, window_y));
            .with_inner_size(glutin::dpi::PhysicalSize::new(window_x, window_y));
        let window = glutin::ContextBuilder::new()
            .with_vsync(true)
            .build_windowed(window_builder, &event_loop)
            .unwrap()
            .make_current()
            .unwrap();
        let gl = glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _);
        gl.enable(DEPTH_TEST);

        let mut renderer = Renderer::new(&gl, window_x/window_y);

        let program = gl.create_program().expect("Cannot create program");

        {   // Shader stuff
            let shader_version = "#version 410";
            let shader_sources = [
                (glow::VERTEX_SHADER, std::fs::read_to_string("src/test.vert")?),
                (glow::FRAGMENT_SHADER, std::fs::read_to_string("src/test.frag")?),
            ];
            let mut shaders = Vec::with_capacity(shader_sources.len());
            for (shader_type, shader_source) in shader_sources.iter() {
                let shader = gl
                    .create_shader(*shader_type)
                    .expect("Cannot create shader");
                gl.shader_source(shader, &format!("{}\n{}", shader_version, shader_source));
                gl.compile_shader(shader);
                if !gl.get_shader_compile_status(shader) {
                    panic!("{}", gl.get_shader_info_log(shader));
                }
                gl.attach_shader(program, shader);
                shaders.push(shader);
            }
            gl.link_program(program);
            if !gl.get_program_link_status(program) {
                panic!("{}", gl.get_program_info_log(program));
            }
            for shader in shaders {
                gl.detach_shader(program, shader);
                gl.delete_shader(shader);
            }
            gl.use_program(Some(program));
        }

        gl.blend_func(SRC_ALPHA, ONE_MINUS_SRC_ALPHA);
        gl.enable(BLEND);
    
        gl.clear_color(0.0, 0.0, 0.0, 1.0);



        let mut application = Application::new();

        let mut held_keys: HashSet<glutin::event::VirtualKeyCode> = HashSet::new();
        let mut lmb = false;
        let mut normalized_cursor_pos = Vec2::new(0.0, 0.0);
        let mut dt = 1.0f64 / 60f64;

        let mut edit = false;

        {
            use glutin::event::{Event, WindowEvent};
            use glutin::event_loop::ControlFlow;

            event_loop.run(move |event, _, control_flow| {
                *control_flow = ControlFlow::Poll;

                let mut cleanup = || {
                    renderer.destroy(&gl);
                    gl.delete_program(program);
                    *control_flow = ControlFlow::Exit;
                };

                match event {
                    Event::LoopDestroyed |
                    Event::WindowEvent {event: WindowEvent::CloseRequested, ..} |
                    Event::WindowEvent {event: WindowEvent::KeyboardInput {
                        input: glutin::event::KeyboardInput { virtual_keycode: Some(glutin::event::VirtualKeyCode::Escape), ..}, ..}, ..}
                    => {
                        cleanup();
                    },


                    Event::MainEventsCleared => {
                        // update
                        let loop_start = SystemTime::now();
                        // draw
                        gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);

                        renderer.clear();
                        
                        gl.uniform_matrix_4_f32_slice(gl.get_uniform_location(program, "projection").as_ref(),
                            false, &projection_mat.to_cols_array());

                        application.draw(&mut renderer, window_x/window_y, normalized_cursor_pos);

                        renderer.present(&gl);
                        
                        window.swap_buffers().unwrap();


                        let loop_end = SystemTime::now();
                        let delta = loop_end.duration_since(loop_start).unwrap().as_secs_f64();
                        let frame_cap = 1.0 / 60.0;
                        // not sure if this handles vsync ay
                        if delta < frame_cap {
                            std::thread::sleep(Duration::from_secs_f64(frame_cap - delta));
                            dt = frame_cap;
                        } else {
                            dt = delta;
                        }
                    }

                    Event::WindowEvent { ref event, .. } => match event {
                        WindowEvent::Resized(physical_size) => {
                            window.resize(*physical_size);
                            window_x = physical_size.width as f32;
                            window_y = physical_size.height as f32;
                            gl.viewport(0, 0, physical_size.width as i32, physical_size.height as i32);
                            println!("aspect ratio: {:?}", window_x / window_y);
                            // level.update_gui(window_x / window_y);

                        }
                        WindowEvent::CloseRequested => {
                            gl.delete_program(program);
                            renderer.destroy(&gl);
                            *control_flow = ControlFlow::Exit
                        }
                        WindowEvent::KeyboardInput {
                            input: glutin::event::KeyboardInput { virtual_keycode: Some(virtual_code), state, .. },
                            ..
                        } => {
                            match state {
                                glutin::event::ElementState::Pressed => held_keys.insert(*virtual_code),
                                glutin::event::ElementState::Released => held_keys.remove(virtual_code),
                            };

                            match (virtual_code, state) {
                                (glutin::event::VirtualKeyCode::Escape, _) => {
                                    gl.delete_program(program);
                                    renderer.destroy(&gl);
                                    *control_flow = ControlFlow::Exit;
                                },
                                (key, glutin::event::ElementState::Pressed) => application.key_press(*key),
                                /*
                                (glutin::event::VirtualKeyCode::E, glutin::event::ElementState::Pressed) => {
                                    level.rotate_forward();
                                },
                                (glutin::event::VirtualKeyCode::Q, glutin::event::ElementState::Pressed) => {
                                    level.rotate_backward();
                                },
                                (glutin::event::VirtualKeyCode::N, glutin::event::ElementState::Pressed) => {
                                    if level_idx > 0 {
                                        level_idx -= 1;
                                    }
                                    level = levels[level_idx].clone();
                                    level.update_gui(window_x/window_y);
                                    println!("{}", level.name)
                                },
                                (glutin::event::VirtualKeyCode::M, glutin::event::ElementState::Pressed) => {
                                    if level_idx < levels.len() - 1 {
                                        level_idx += 1;
                                    }
                                    level = levels[level_idx].clone();
                                    level.update_gui(window_x/window_y);
                                    println!("{}", level.name)
                                },
                                (glutin::event::VirtualKeyCode::P, glutin::event::ElementState::Pressed) => {
                                    edit = match edit {
                                        true => {
                                            println!("edit off");
                                            false
                                        },
                                        false => {
                                            println!("edit on");
                                            true
                                        }
                                    }
                                },
                                */
                            _ => (),
                        }},
                        WindowEvent::MouseInput {
                            button: glutin::event::MouseButton::Right,
                            state: glutin::event::ElementState::Pressed,
                            ..
                        } => {
                            application.rmb(normalized_cursor_pos);
                            //game.apply_command(InputCommand::EatGun);
                        }
                        WindowEvent::MouseInput {
                            button: glutin::event::MouseButton::Left,
                            state,
                            ..
                        } => {
                            if *state == glutin::event::ElementState::Pressed {
                                lmb = true;
                                application.lmb(normalized_cursor_pos);
                            } else {
                                lmb = false;
                            }
                        },
                        WindowEvent::CursorMoved {
                            position: pos,
                            ..
                        } => {
                            normalized_cursor_pos = Vec2::new(
                                // pos.x as f32 / window_x * window_x / window_y, 
                                pos.x as f32 / window_x, 
                                pos.y as f32 / window_y);
                        },
                        _ => (),
                    },
                    _ => (),
                }
            });
        }
    }
}