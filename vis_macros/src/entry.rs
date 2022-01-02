use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::parse::Parser;
pub(crate) fn main(args: TokenStream, item: TokenStream) -> TokenStream {
    let mut input: syn::ItemFn = match syn::parse(item.clone()) {
        Ok(it) => it,
        Err(e) => {
            item.clone().extend(TokenStream::from(e.into_compile_error()));
            return item
        }
    };
    
    let body = &input.block;
    
    let test = quote! {
        {
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
                        #body
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
            })
        }
    };
    /*
    let test = quote! {
        {
            env_logger::init();
            let event_loop = EventLoop::new();
            let window = WindowBuilder::new()
                .with_title("Sorting VIsualizer")
                .build(&event_loop)
                .unwrap();
            
            let mut state = pollster::block_on(vis_core::surface::State::new(&window, &vis_arr.vertices, &vis_arr.indices));
            
            event_loop.run(move |event, _, control_flow| {
                *control_flow = ControlFlow::Wait;
            })
        }
    };
    */

    let brace_token = input.block.brace_token;

    input.block = syn::parse2(quote! {
        #test
    }).expect("Parsing Failure");

    input.block.brace_token = brace_token;

    let result = quote! {
        #input
    };
    
    result.into()
}