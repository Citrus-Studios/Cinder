use std::{thread::sleep, time::Duration};

use cinder::zeroed_vec;
use sdl2::{
    event::{Event, WindowEvent},
    keyboard::Keycode,
    sys::SDL_Vulkan_GetInstanceExtensions,
};

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .vulkan()
        .build()
        .unwrap();

    let mut count: u32 = 0;
    unsafe { SDL_Vulkan_GetInstanceExtensions(window.raw(), &mut count, std::ptr::null_mut()) };

    let mut extensions = unsafe { zeroed_vec(count as usize) };
    unsafe { SDL_Vulkan_GetInstanceExtensions(window.raw(), &mut count, extensions.as_mut_ptr()) };

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut fps = Duration::from_nanos(016666);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::Window { win_event, .. } => match win_event {
                    WindowEvent::Enter => fps = Duration::from_nanos(016666),
                    WindowEvent::Leave => fps = Duration::from_nanos(100000),
                    _ => {}
                },
                _ => {
                    println!("{:?}", event);
                }
            }
        }

        sleep(fps);
    }
}
