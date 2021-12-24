#![feature(proc_macro)]

extern crate vis_macros;
extern crate vis_core;

//use vis_macros::init;
use vis_core::vis_array;
use vis_core::surface;
use winit::*;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    window::Window,
};
use rand::{thread_rng, Rng};

fn create_rand_array(size: u16) -> Vec<u16> {
    let mut arr = Vec::<u16>::new();
    for i in 0..size {
        arr.push((i as u16) + 1);
    }
    let mut rng = thread_rng();
    for i in 0..size {
        let s: usize = rng.gen_range(0..size-1).into();
        let temp = arr[s];
        arr[s] = arr[i as usize];
        arr[i as usize] = temp;
    }
    
    arr
}
#[vis_macros::main]
fn selection_sort(arr: &mut [u16], vis_arr: &mut vis_array::VisualArray) {
    let mut min_idx;
    for i in 0..arr.len() {
        vis_arr.change_color(&mut state, &mut window, i as u32, vis_array::Color::Green);
        //println!("{}", i);
        min_idx = i;
        for j in (i+1)..arr.len() {
            if arr[min_idx] > arr[j] {
                min_idx = j;
            }
        }
        let temp = arr[min_idx];
        arr[min_idx] = arr[i];
        arr[i] = temp;
        vis_arr.swap(&mut state, &mut window, min_idx as u32, i as u32);
        
        vis_arr.change_color(&mut state, &mut window, i as u32, vis_array::Color::White);
    }
    /*
    for i in 1..arr.len() {
        let key = arr[i];

        let mut j = i-1;
        while j >= 0 && key < arr[j] {
            vis_arr.swap(state, &window, (j+1) as u32, j as u32);
            j = j - 1;
        }    
        vis_arr.swap(state, &window, (j+1) as u32, i as u32);
    }
    */
}
fn main() {
    let mut arr = create_rand_array(100);
    let mut vis_arr = vis_array::VisualArray::new(&mut arr);
    vis_arr = vis_array::VisualArray::new(&mut arr);
    selection_sort(&mut arr, &mut vis_arr);
    //init!(selection_sort(&mut arr, &mut vis_arr));
    /*
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Sorting Visualizer")
        .build(&event_loop)
        .unwrap();
    
    let mut state = pollster::block_on(vis_core::surface::State::new(&window, &vis_arr.vertices, &vis_arr.indices));

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

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
                selection_sort(&mut state, &window, &mut arr, &mut vis_arr);
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
                _ => {}
            },
            _ => {}
        };
    });
    */
}


