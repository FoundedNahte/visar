
extern crate vis_macros;
extern crate vis_core;

//use vis_macros::init;
use vis_core::vis_array;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn selection_sort(state: &mut vis_core::surface::State, vis_arr: &mut vis_core::vis_array::VisualArray) {
    let mut min_idx;
    for i in 0..vis_arr.size {
        vis_arr.change_color(state, i as u32, vis_array::Color::Green);
        min_idx = i;
        for j in (i+1)..vis_arr.size {
            if vis_arr.access(min_idx) > vis_arr.access(j) {
                min_idx = j;
            }
        }
        let temp = vis_arr.access(min_idx);
        let value = vis_arr.access(i); 
        vis_arr.change(min_idx, value);
        vis_arr.change(i, temp);
        vis_arr.swap(state, min_idx as u32, i as u32);

        vis_arr.change_color(state, i as u32, vis_array::Color::White);
    }
    vis_arr.finish(state);
}

fn bubble_sort(state: &mut vis_core::surface::State, vis_arr: &mut vis_core::vis_array::VisualArray) {
    for i in 0..vis_arr.size-1 {
        vis_arr.change_color(state, i as u32, vis_array::Color::Green);
        for j in 0..vis_arr.size-i-1 {
            if vis_arr.access(j) > vis_arr.access(j+1) {
                let temp = vis_arr.access(j);
                let temp2 = vis_arr.access(j+1);
                vis_arr.change(j, temp2);
                vis_arr.change(j+1, temp);
                vis_arr.swap(state, j as u32, (j+1) as u32);    
            }
        }
        vis_arr.change_color(state, i as u32, vis_array::Color::White);
    }
}
fn main() {
    let mut vis_arr = vis_array::VisualArray::new(vis_array::create_rand_array(100));
    //selection_sort(&mut vis_arr);
    //init!(selection_sort(&mut arr, &mut vis_arr));
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Sorting Visualizer")
        .build(&event_loop)
        .unwrap();
    
    let mut state = pollster::block_on(vis_core::surface::State::new(&window, &vis_arr.vertices, &vis_arr.indices));

    let mut sorted: bool = false;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

         match event {
            Event::RedrawRequested(_) => {
                match state.render() {
                    Ok(_) => {},
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                    input: KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                    ..
                } => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(physical_size) => {
                    state.resize(*physical_size);
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    state.resize(**new_inner_size);
                }
                WindowEvent::KeyboardInput {
                    input: KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Space),
                        ..
                    },
                    ..
                } => { 
                    if !sorted {
                        selection_sort(&mut state, &mut vis_arr);
                        sorted = true;
                    } else {
                        vis_arr.update(vis_array::create_rand_array(100));
                        window.request_redraw();
                        bubble_sort(&mut state, &mut vis_arr);
                    }
                }
                _ => {}
            },
            _ => {}
        };
    });
}


