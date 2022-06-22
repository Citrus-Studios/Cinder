use std::{thread::sleep, time::Duration};

use cinder::zeroed_vec;
use sdl2::{event::Event, keyboard::Keycode, sys::SDL_Vulkan_GetInstanceExtensions};

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

    let fps = Duration::from_nanos(016666);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        sleep(fps);
    }
}
