use std::time::Duration;

use cinder::vulkan::safe::{instance_items::instance::InstanceBuilder, device_items::physical_device::PhysicalDevice};
// use sdl2::{sys::SDL_Vulkan_GetInstanceExtensions, keyboard::Keycode, event::Event};
use cinder::zeroed_vec;

fn main() {
    // let sdl_context = sdl2::init().unwrap();
    // let video_subsystem = sdl_context.video().unwrap();

    // let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
    //     .position_centered()
    //     .vulkan()
    //     .build()
    //     .unwrap();

    // let mut count:u32 = 0;
    // unsafe { SDL_Vulkan_GetInstanceExtensions(window.raw(), &mut count, std::ptr::null_mut()) };

    // let mut extensions = unsafe { zeroed_vec(count as usize) };
    // unsafe { SDL_Vulkan_GetInstanceExtensions(window.raw(), &mut count, extensions.as_mut_ptr()) };

    // let extensions = extensions.into_iter().map(|x| {
    //     unsafe { std::ffi::CStr::from_ptr(x).to_str().unwrap() }
    // }).collect();
    // println!("{:?}", extensions);

    let instance = InstanceBuilder::new()
        .application_name("Cinder")
        // .extensions(extensions)
        // .layers(vec!["VK_LAYER_KHRONOS_validation", "VK_LAYER_LUNARG_api_dump"])
        .engine_name("None")
        .build();

    // let _physical_device = PhysicalDevice::new(instance, &window).pick_best_device();

    // let mut event_pump = sdl_context.event_pump().unwrap();

    // 'running: loop {
    //     println!("hi");
    //     for event in event_pump.poll_iter() {
    //         match event {
    //             Event::Quit { .. }
    //             | Event::KeyDown {
    //                 keycode: Some(Keycode::Escape),
    //                 ..
    //             } => break 'running,
    //             _ => {}
    //         }
    //     }

    //     std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    //     // The rest of the game loop goes here...
    // }
}
