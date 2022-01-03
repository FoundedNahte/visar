
extern crate vis_macros;
extern crate vis_core;

//use vis_macros::init;
use std::collections::HashMap;
use vis_core::vis_array;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
//
//
//
fn odd_even_sort(state: &mut vis_core::surface::State, vis_arr: &mut vis_core::vis_array::VisualArray) {
    let mut sorted = false;
    while !sorted {
        sorted = true;
        let mut i = 1;
        while i < vis_arr.size-1 {
            if vis_arr.access(i) > vis_arr.access(i+1) {
                vis_arr.swap(state, i, i+1);
                sorted = false;
            }
            i += 2;
        }
        i = 0;
        while i < vis_arr.size-1 {
            if vis_arr.access(i) > vis_arr.access(i+1) {
                vis_arr.swap(state, i, i+1);
                sorted = false;
            }
            i += 2;
        }
    }
}
// ******************************************************************************************
// SHELL SORT
//
fn shell_sort(state: &mut vis_core::surface::State, vis_arr: &mut vis_core::vis_array::VisualArray) {
    let incs = [1391376, 463792, 198768, 86961, 33936,
                        13776, 4592, 1968, 861, 336, 112, 48, 21, 
                        7, 3, 1];
    for k in 0..16 {
        let mut h = incs[k];
        let mut i = h;
        while i < vis_arr.size {
            let v = vis_arr.access(i);
            let mut j = i;
            while j >= h {
                let temp = vis_arr.access(j-h);
                if(temp > v) {
                    vis_arr.set(state, j, temp);
                    j -= h;
                } else {
                    break;
                }
            }
            
            vis_arr.set(state, j, v);
            i += 1;
        }
    }

}

// ************************************************************************************************************
// RADIX SORT (WITH COUNTING SORT)
//

fn count_sort(state: &mut vis_core::surface::State, vis_arr: &mut vis_core::vis_array::VisualArray, exp: u16) {
    let mut output: Vec<u16> = vec![0; vis_arr.size];
    let mut count = [0; 10];
    
    for i in 0..vis_arr.size {
        count[((vis_arr.access(i) / exp) % 10) as usize] += 1;
    }
    
    for i in 1..10 {
        count[i] += count[i - 1];
    }
    
    let mut j = vis_arr.size - 1;
    while j >= 0 {
        output[count[((vis_arr.access(j) / exp) % 10) as usize] - 1 as usize] = vis_arr.access(j);
        count[((vis_arr.access(j) / exp) % 10) as usize] -= 1;
        if j == 0 {
            break;
        } else {
            j -= 1;
        }
    }
    
    for i in 0..vis_arr.size {
        vis_arr.set(state, i, output[i]);
    }

}
fn radix_sort(state: &mut vis_core::surface::State, vis_arr: &mut vis_core::vis_array::VisualArray) {
    let m = match vis_arr.array.iter().max() {
        Some(value) => value.clone(),
        None => panic!("Empty Array"),
    };
    
    let mut exp = 1;
    while m / exp > 0 {
        count_sort(state, vis_arr, exp);
        exp *= 10;
    }

}

// *****************************************************************************************
// INSERTION SORT
//
fn insertion_sort(state: &mut vis_core::surface::State, vis_arr: &mut vis_core::vis_array::VisualArray) {
    for i in 1..vis_arr.size {
        let key = vis_arr.access(i).clone();
        let mut j = i - 1;
        let mut temp = vis_arr.access(j).clone();
        while j >= 0 && temp > key {
            vis_arr.swap(state, j, (j+1));
            if j == 0 {
                break;
            } else {
                j -= 1;
                temp = vis_arr.access(j).clone();
            }
        }
    }
}


// ******************************************************************************************
// SELECTION SORT
//
fn selection_sort(state: &mut vis_core::surface::State, vis_arr: &mut vis_core::vis_array::VisualArray) {
    let mut min_idx;
    for i in 0..vis_arr.size {
        vis_arr.change_color(state, i, vis_array::Color::Green);
        min_idx = i;
        for j in (i+1)..vis_arr.size {
            if vis_arr.access(min_idx) > vis_arr.access(j) {
                min_idx = j;
            }
        }
        vis_arr.swap(state, min_idx, i);

        vis_arr.change_color(state, i, vis_array::Color::White);
    }
    vis_arr.finish(state);
}

// ********************************************************************************************
// BUBBLE SORT
//
fn bubble_sort(state: &mut vis_core::surface::State, vis_arr: &mut vis_core::vis_array::VisualArray) {
    for i in 0..vis_arr.size-1 {
        vis_arr.change_color(state, i, vis_array::Color::Green);
        for j in 0..vis_arr.size-i-1 {
            if vis_arr.access(j) > vis_arr.access(j+1) {
                vis_arr.swap(state, j, j+1);
            }        
        }
        vis_arr.change_color(state, i, vis_array::Color::White);
    }
}

// *********************************************************************************************
// DRIVER FUNCTION
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

    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .build()
        .unwrap(); 

    let mut sorted: bool = false;

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
                        //handle = rt.spawn(selection_sort(&state, &vis_arr));
                        radix_sort(&mut state, &mut vis_arr);
                        sorted = true;
                    } else {
                        vis_arr.update(&mut state, &vis_array::create_rand_array(100));
                        window.request_redraw();
                        radix_sort(&mut state, &mut vis_arr);
                        //handle = rt.spawn(bubble_sort(&state, &vis_arr));
                    }
                    //rt.block_on(handle);
                }
                _ => {}
            },
            _ => {}
        };
    });
}


