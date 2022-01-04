
extern crate visar_core;

use clap::Parser;
use visar_core::vis_array;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
// *****************************************************************************************
// QUICK SORT
//
fn partition(state: &mut visar_core::surface::State, vis_arr: &mut visar_core::vis_array::VisualArray, low: i32, high: i32) -> i32 {
    let pivot = vis_arr.access(high as usize);
    let mut i = low - 1;
    for j in low..high {
        if vis_arr.access(j as usize) <= pivot {
            i += 1;
            vis_arr.swap(state, i as usize, j as usize);
        }
    }
    vis_arr.swap(state, (i+1) as usize, high as usize);

    return i + 1;
}

fn quick_sort(state: &mut visar_core::surface::State, vis_arr: &mut visar_core::vis_array::VisualArray, low: i32, high: i32) {
    if low < high {
        let pi = partition(state, vis_arr, low, high);

        quick_sort(state, vis_arr, low, pi - 1);
        quick_sort(state, vis_arr, pi + 1, high);
    }
}

// *****************************************************************************************
// HEAP SORT
//
fn heap_sort(state: &mut visar_core::surface::State, vis_arr: &mut visar_core::vis_array::VisualArray) {
    let mut n = vis_arr.size;
    let mut i = n / 2;
    
    loop {
        if i > 0 {
            i -= 1;
        } else {
            n -= 1;
            if n == 0 {
                return;
            }
            vis_arr.swap(state, 0, n as usize);
            
        }
        let mut parent = i;
        let mut child = (i * 2) + 1;
        
        while child < n {
            if (child + 1) < n && vis_arr.access(child + 1) > vis_arr.access(child) {
                child += 1;
            }
            if vis_arr.access(child) > vis_arr.access(parent) {
                vis_arr.swap(state, parent, child);
                parent = child;
                child = (parent * 2) + 1;
            } else {
                break;
            }
        }
    }
}
// *****************************************************************************************
// COMB SORT
//
fn get_gap(gap: i32) -> i32 {
    let mut temp = gap.clone();
    temp = (temp * 10) / 13;
    if gap < 1 {
        return 1;
    }
    temp
}
fn comb_sort(state: &mut visar_core::surface::State, vis_arr: &mut visar_core::vis_array::VisualArray, n: i32) {
    let mut gap = n.clone();

    let mut swapped = true;

    while  gap != 1 || swapped {
        gap = get_gap(gap);
        swapped = false;
        for i in 0..n-gap {
            if vis_arr.access(i as usize) > vis_arr.access((i + gap) as usize) {
                vis_arr.swap(state, i as usize, (i + gap) as usize);
                swapped = true;
            }
        }
    }
}

// *****************************************************************************************
// MERGE SORT
//
fn merge(state: &mut visar_core::surface::State, vis_arr: &mut visar_core::vis_array::VisualArray, start: u16, mid: u16, end: u16) {
    let mut temp: Vec<u16> = vec![0; (end - start + 1) as usize];
    
    let mut i = start;
    let mut j = mid + 1;
    let mut k = 0;
    
    while i <= mid && j <= end {
        if vis_arr.access(i as usize) <= vis_arr.access(j as usize) {
            temp[k] = vis_arr.access(i as usize);
            k += 1;
            i += 1;
        } else {
            temp[k] = vis_arr.access(j as usize);
            k += 1;
            j += 1;
        }
    }
    
    while i <= mid {
        temp[k] = vis_arr.access(i as usize);
        k += 1;
        i += 1;
    }
    
    while j <= end {
        temp[k] = vis_arr.access(j as usize);
        k += 1;
        j += 1;
    }
    i = start;
    while i <= end {
        vis_arr.set(state, i as usize, temp[(i - start) as usize]);
        i += 1;
    }
    
}

fn merge_sort(state: &mut visar_core::surface::State, vis_arr: &mut visar_core::vis_array::VisualArray, start: u16, end: u16) {
    if start < end {
        let mid = (start + end) / 2;
        merge_sort(state, vis_arr, start, mid);
        merge_sort(state, vis_arr, mid+1, end);
        merge(state, vis_arr, start, mid, end);
    }
}
// ******************************************************************************************
// ODD EVEN SORT
//
fn odd_even_sort(state: &mut visar_core::surface::State, vis_arr: &mut visar_core::vis_array::VisualArray) {
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
fn shell_sort(state: &mut visar_core::surface::State, vis_arr: &mut visar_core::vis_array::VisualArray) {
    let incs = [1391376, 463792, 198768, 86961, 33936,
                        13776, 4592, 1968, 861, 336, 112, 48, 21, 
                        7, 3, 1];
    for k in 0..16 {
        let h = incs[k];
        let mut i = h;
        while i < vis_arr.size {
            let v = vis_arr.access(i);
            let mut j = i;
            while j >= h {
                let temp = vis_arr.access(j-h);
                if temp > v {
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

fn count_sort(state: &mut visar_core::surface::State, vis_arr: &mut visar_core::vis_array::VisualArray, exp: u16) {
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
fn radix_sort(state: &mut visar_core::surface::State, vis_arr: &mut visar_core::vis_array::VisualArray) {
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
fn insertion_sort(state: &mut visar_core::surface::State, vis_arr: &mut visar_core::vis_array::VisualArray) {
    for i in 1..vis_arr.size {
        let key = vis_arr.access(i).clone();
        let mut j = i - 1;
        let mut temp = vis_arr.access(j).clone();
        while j >= 0 && temp > key {
            vis_arr.swap(state, j, j+1);
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
fn selection_sort(state: &mut visar_core::surface::State, vis_arr: &mut visar_core::vis_array::VisualArray) {
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
fn bubble_sort(state: &mut visar_core::surface::State, vis_arr: &mut visar_core::vis_array::VisualArray) {
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
#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Config {
    // Which algorithm to use
    #[clap(short, long)]
    algo: String,

    // Size of the array
    #[clap(short, long, default_value_t = 100)]
    size: u16,
}

fn main() {
    let args = Config::parse();

    let mut vis_arr = vis_array::VisualArray::new(vis_array::create_rand_array(args.size));

    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Sorting Visualizer")
        .build(&event_loop)
        .unwrap();
    
    let mut state = pollster::block_on(visar_core::surface::State::new(&window, &vis_arr.vertices, &vis_arr.indices));

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
                        let temp = &args.algo[..];
                        match temp {
                            "selection" => { selection_sort(&mut state, &mut vis_arr); },
                            "oddeven" => { odd_even_sort(&mut state, &mut vis_arr); },
                            "radix" => { radix_sort(&mut state, &mut vis_arr); },
                            "insertion" => { insertion_sort(&mut state, &mut vis_arr); },
                            "bubble" => { bubble_sort(&mut state, &mut vis_arr); },
                            "shell" => { shell_sort(&mut state, &mut vis_arr); },
                            "merge" => { merge_sort(&mut state, &mut vis_arr, 0, args.size-1); },
                            "comb" => { comb_sort(&mut state, &mut vis_arr, args.size as i32); },
                            "heap" => { heap_sort(&mut state, &mut vis_arr); },
                            "quick" => { quick_sort(&mut state, &mut vis_arr, 0, (args.size-1) as i32); },
                            _ => { panic!("Algorithm not available"); },
                        }
                        sorted = true;
                    } else {
                        vis_arr.update(&mut state, &vis_array::create_rand_array(args.size));
                        window.request_redraw();
                        let temp = &args.algo[..];
                        match temp {
                            "selection" => { selection_sort(&mut state, &mut vis_arr); },
                            "oddeven" => { odd_even_sort(&mut state, &mut vis_arr); },
                            "radix" => { radix_sort(&mut state, &mut vis_arr); },
                            "insertion" => { insertion_sort(&mut state, &mut vis_arr); },
                            "bubble" => { bubble_sort(&mut state, &mut vis_arr); },
                            "shell" => { shell_sort(&mut state,&mut vis_arr); },
                            "merge" => { merge_sort(&mut state, &mut vis_arr, 0, args.size-1); },
                            "comb" => { comb_sort(&mut state, &mut vis_arr, args.size as i32); },
                            "heap" => { heap_sort(&mut state, &mut vis_arr); },
                            "quick" => { quick_sort(&mut state, &mut vis_arr, 0, (args.size-1) as i32); },
                            _ => { panic!("Algorithm not available"); },
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        };
    });
}


