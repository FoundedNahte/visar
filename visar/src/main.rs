//
// MAIN SOURCE CODE THAT INCLUDES ALGORITHM FUNCTIONS AND DRIVER FUNCTION
//
extern crate visar_audio;
extern crate visar_core;

use clap::Parser;
use std::thread;
use visar_audio::sound::{AudioController, Format};
use visar_core::vis_array;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
// *****************************************************************************************
// QUICK SORT
//
fn partition(handles: &mut Handles, low: i32, high: i32) -> i32 {
    let pivot = handles
        .vis_arr
        .access(&handles.command_sender, high as usize);
    let mut i = low - 1;
    for j in low..high {
        if handles.vis_arr.access(&handles.command_sender, j as usize) <= pivot {
            i += 1;
            handles
                .vis_arr
                .swap(&mut handles.state, i as usize, j as usize);
        }
    }
    handles
        .vis_arr
        .swap(&mut handles.state, (i + 1) as usize, high as usize);

    return i + 1;
}

fn quick_sort(handles: &mut Handles, low: i32, high: i32) {
    if low < high {
        let pi = partition(handles, low, high);

        quick_sort(handles, low, pi - 1);
        quick_sort(handles, pi + 1, high);
    }
    if low == 0 {
        handles.vis_arr.finish(&handles.command_sender);
        return;
    }
}

// *****************************************************************************************
// HEAP SORT}
//
fn heap_sort(handles: &mut Handles) {
    let mut n = handles.vis_arr.size;
    let mut i = n / 2;

    loop {
        if handles.vis_arr.array[0] == 1 {
            break;
        }
        if i > 0 {
            i -= 1;
        } else {
            n -= 1;
            if n == 0 {
                return;
            }
            handles.vis_arr.swap(&mut handles.state, 0, n as usize);
        }
        let mut parent = i;
        let mut child = (i * 2) + 1;

        while child < n {
            if (child + 1) < n
                && handles.vis_arr.access(&handles.command_sender, child + 1)
                    > handles.vis_arr.access(&handles.command_sender, child)
            {
                child += 1;
            }
            if handles.vis_arr.access(&handles.command_sender, child)
                > handles.vis_arr.access(&handles.command_sender, parent)
            {
                handles.vis_arr.swap(&mut handles.state, parent, child);
                parent = child;
                child = (parent * 2) + 1;
            } else {
                break;
            }
        }
    }
    handles.vis_arr.finish(&handles.command_sender);
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
fn comb_sort(handles: &mut Handles, n: i32) {
    let mut gap = n.clone();

    let mut swapped = true;

    while gap != 1 || swapped {
        gap = get_gap(gap);
        swapped = false;
        for i in 0..n - gap {
            if handles.vis_arr.access(&handles.command_sender, i as usize)
                > handles
                    .vis_arr
                    .access(&handles.command_sender, (i + gap) as usize)
            {
                handles
                    .vis_arr
                    .swap(&mut handles.state, i as usize, (i + gap) as usize);
                swapped = true;
            }
        }
    }
    handles.vis_arr.finish(&handles.command_sender);
}

// *****************************************************************************************
// MERGE SORT
//
fn merge(handles: &mut Handles, start: u16, mid: u16, end: u16) {
    let mut temp: Vec<u16> = vec![0; (end - start + 1) as usize];

    let mut i = start;
    let mut j = mid + 1;
    let mut k = 0;

    while i <= mid && j <= end {
        if handles.vis_arr.access(&handles.command_sender, i as usize)
            <= handles.vis_arr.access(&handles.command_sender, j as usize)
        {
            temp[k] = handles.vis_arr.access(&handles.command_sender, i as usize);
            k += 1;
            i += 1;
        } else {
            temp[k] = handles.vis_arr.access(&handles.command_sender, j as usize);
            k += 1;
            j += 1;
        }
    }

    while i <= mid {
        temp[k] = handles.vis_arr.access(&handles.command_sender, i as usize);
        k += 1;
        i += 1;
    }

    while j <= end {
        temp[k] = handles.vis_arr.access(&handles.command_sender, j as usize);
        k += 1;
        j += 1;
    }
    i = start;
    while i <= end {
        handles.vis_arr.set(
            &handles.command_sender,
            &mut handles.state,
            i as usize,
            temp[(i - start) as usize],
        );
        i += 1;
    }
}

fn merge_sort(handles: &mut Handles, start: u16, end: u16) {
    if start < end {
        let mid = (start + end) / 2;
        merge_sort(handles, start, mid);
        merge_sort(handles, mid + 1, end);
        merge(handles, start, mid, end);
    }
    if start == 0 {
        handles.vis_arr.finish(&handles.command_sender);
    }
}
// ******************************************************************************************
// ODD EVEN SORT
//
fn odd_even_sort(handles: &mut Handles) {
    let mut sorted = false;
    while !sorted {
        sorted = true;
        let mut i = 1;
        while i < handles.vis_arr.size - 1 {
            if handles.vis_arr.access(&handles.command_sender, i)
                > handles.vis_arr.access(&handles.command_sender, i + 1)
            {
                handles.vis_arr.swap(&mut handles.state, i, i + 1);
                sorted = false;
            }
            i += 2;
        }
        i = 0;
        while i < handles.vis_arr.size - 1 {
            if handles.vis_arr.access(&handles.command_sender, i)
                > handles.vis_arr.access(&handles.command_sender, i + 1)
            {
                handles.vis_arr.swap(&mut handles.state, i, i + 1);
                sorted = false;
            }
            i += 2;
        }
    }
    handles.vis_arr.finish(&handles.command_sender);
}
// ******************************************************************************************
// SHELL SORT
//
fn shell_sort(handles: &mut Handles) {
    let incs = [
        1391376, 463792, 198768, 86961, 33936, 13776, 4592, 1968, 861, 336, 112, 48, 21, 7, 3, 1,
    ];
    for k in 0..16 {
        let h = incs[k];
        let mut i = h;
        while i < handles.vis_arr.size {
            let v = handles.vis_arr.access(&handles.command_sender, i);
            let mut j = i;
            while j >= h {
                let temp = handles.vis_arr.access(&handles.command_sender, j - h);
                if temp > v {
                    handles
                        .vis_arr
                        .set(&handles.command_sender, &mut handles.state, j, temp);
                    j -= h;
                } else {
                    break;
                }
            }

            handles
                .vis_arr
                .set(&handles.command_sender, &mut handles.state, j, v);
            i += 1;
        }
    }
    handles.vis_arr.finish(&handles.command_sender);
}

// ************************************************************************************************************
// RADIX SORT (WITH COUNTING SORT)
//

fn count_sort(handles: &mut Handles, exp: u16) {
    let mut output: Vec<u16> = vec![0; handles.vis_arr.size];
    let mut count = [0; 10];

    for i in 0..handles.vis_arr.size {
        count[((handles.vis_arr.access(&handles.command_sender, i) / exp) % 10) as usize] += 1;
    }

    for i in 1..10 {
        count[i] += count[i - 1];
    }

    let mut j = handles.vis_arr.size - 1;
    while j >= 0 {
        output[count[((handles.vis_arr.access(&handles.command_sender, j) / exp) % 10) as usize]
            - 1 as usize] = handles.vis_arr.access(&handles.command_sender, j);
        count[((handles.vis_arr.access(&handles.command_sender, j) / exp) % 10) as usize] -= 1;
        if j == 0 {
            break;
        } else {
            j -= 1;
        }
    }

    for i in 0..handles.vis_arr.size {
        handles
            .vis_arr
            .set(&handles.command_sender, &mut handles.state, i, output[i]);
    }
}
fn radix_sort(handles: &mut Handles) {
    let m = match handles.vis_arr.array.iter().max() {
        Some(value) => value.clone(),
        None => panic!("Empty Array"),
    };

    let mut exp = 1;
    while m / exp > 0 {
        count_sort(handles, exp);
        exp *= 10;
    }
    handles.vis_arr.finish(&handles.command_sender);
}

// *****************************************************************************************
// INSERTION SORT
//
fn insertion_sort(handles: &mut Handles) {
    for i in 1..handles.vis_arr.size {
        let key = handles.vis_arr.access(&handles.command_sender, i).clone();
        let mut j = i - 1;
        let mut temp = handles.vis_arr.access(&handles.command_sender, j).clone();
        while j >= 0 && temp > key {
            handles.vis_arr.swap(&mut handles.state, j, j + 1);
            if j == 0 {
                break;
            } else {
                j -= 1;
                temp = handles.vis_arr.access(&handles.command_sender, j).clone();
            }
        }
    }
    handles.vis_arr.finish(&handles.command_sender);
}

// ******************************************************************************************
// SELECTION SORT
//
fn selection_sort(handles: &mut Handles) {
    let mut min_idx;
    for i in 0..handles.vis_arr.size {
        handles
            .vis_arr
            .change_color(&mut handles.state, i, vis_array::Color::Green);
        min_idx = i;
        for j in (i + 1)..handles.vis_arr.size {
            if handles.vis_arr.access(&handles.command_sender, min_idx)
                > handles.vis_arr.access(&handles.command_sender, j)
            {
                min_idx = j;
            }
        }
        handles.vis_arr.swap(&mut handles.state, min_idx, i);

        handles
            .vis_arr
            .change_color(&mut handles.state, i, vis_array::Color::White);
    }
    handles.vis_arr.finish(&handles.command_sender);
}

// ********************************************************************************************
// BUBBLE SORT
//
fn bubble_sort(handles: &mut Handles) {
    for i in 0..handles.vis_arr.size - 1 {
        handles
            .vis_arr
            .change_color(&mut handles.state, i, vis_array::Color::Green);
        for j in 0..handles.vis_arr.size - i - 1 {
            if handles.vis_arr.access(&handles.command_sender, j)
                > handles.vis_arr.access(&handles.command_sender, j + 1)
            {
                handles.vis_arr.swap(&mut handles.state, j, j + 1);
            }
        }
        handles
            .vis_arr
            .change_color(&mut handles.state, i, vis_array::Color::White);
    }
    handles.vis_arr.finish(&handles.command_sender);
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

struct Handles {
    pub state: visar_core::surface::State,
    pub vis_arr: vis_array::VisualArray,
    pub command_sender: Option<crossbeam::channel::Sender<visar_audio::sound::Message>>,
}
fn main() {
    let args = Config::parse();

    let vis_arr = vis_array::VisualArray::new(vis_array::create_rand_array(args.size));

    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Sorting Visualizer")
        .build(&event_loop)
        .unwrap();

    let state = pollster::block_on(visar_core::surface::State::new(
        &window,
        &vis_arr.vertices,
        &vis_arr.indices,
    ));

    let mut audio_controller: Option<AudioController> = match AudioController::new() {
        Ok(ac) => Some(ac),
        Err(e) => {
            eprintln!("{}", e);
            None
        }
    };

    let mut handles;
    if let Some(_ac) = &audio_controller {
        handles = Handles {
            state,
            vis_arr,
            command_sender: Some(audio_controller.as_ref().unwrap().command_sender.clone()),
        }
    } else {
        handles = Handles {
            state,
            vis_arr,
            command_sender: None,
        }
    }

    match audio_controller {
        Some(ac) => {
            thread::spawn(move || {
                match ac.sample_format {
                    Format::F32format => AudioController::run::<f32>(ac),
                    Format::I16format => AudioController::run::<i16>(ac),
                    Format::U16format => AudioController::run::<u16>(ac),
                }
                .expect("Error starting the audio stream");
            });
        }
        None => {}
    }

    let mut sorted: bool = false;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::RedrawRequested(_) => match handles.state.render() {
                Ok(_) => {}
                Err(wgpu::SurfaceError::Lost) => handles.state.resize(handles.state.size),
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                Err(e) => eprintln!("{:?}", e),
            },
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(physical_size) => {
                    handles.state.resize(*physical_size);
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    handles.state.resize(**new_inner_size);
                }
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Space),
                            ..
                        },
                    ..
                } => {
                    if !sorted {
                        let temp = &args.algo[..];
                        match temp {
                            "selection" => {
                                selection_sort(&mut handles);
                            }
                            "oddeven" => {
                                odd_even_sort(&mut handles);
                            }
                            "radix" => {
                                radix_sort(&mut handles);
                            }
                            "insertion" => {
                                insertion_sort(&mut handles);
                            }
                            "bubble" => {
                                bubble_sort(&mut handles);
                            }
                            "shell" => {
                                shell_sort(&mut handles);
                            }
                            "merge" => {
                                merge_sort(&mut handles, 0, args.size - 1);
                            }
                            "comb" => {
                                comb_sort(&mut handles, args.size as i32);
                            }
                            "heap" => {
                                heap_sort(&mut handles);
                            }
                            "quick" => {
                                quick_sort(&mut handles, 0, (args.size - 1) as i32);
                            }
                            _ => {
                                panic!("Algorithm not available");
                            }
                        }
                        sorted = true;
                    } else {
                        handles
                            .vis_arr
                            .update(&mut handles.state, &vis_array::create_rand_array(args.size));
                        window.request_redraw();
                        let temp = &args.algo[..];
                        match temp {
                            "selection" => {
                                selection_sort(&mut handles);
                            }
                            "oddeven" => {
                                odd_even_sort(&mut handles);
                            }
                            "radix" => {
                                radix_sort(&mut handles);
                            }
                            "insertion" => {
                                insertion_sort(&mut handles);
                            }
                            "bubble" => {
                                bubble_sort(&mut handles);
                            }
                            "shell" => {
                                shell_sort(&mut handles);
                            }
                            "merge" => {
                                merge_sort(&mut handles, 0, args.size - 1);
                            }
                            "comb" => {
                                comb_sort(&mut handles, args.size as i32);
                            }
                            "heap" => {
                                heap_sort(&mut handles);
                            }
                            "quick" => {
                                quick_sort(&mut handles, 0, (args.size - 1) as i32);
                            }
                            _ => {
                                panic!("Algorithm not available");
                            }
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        };
    });
}
