#[test]
fn triangle() {
    use crate::vulkan::safe::{instance_items::instance::InstanceBuilder, device_items::physical_device::PhysicalDevice};

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .vulkan()
        .build()
        .unwrap();

    let instance = InstanceBuilder::new()
        .application_name("Cinder")
        .engine_name("None")
        .build();

    let _physical_device = PhysicalDevice::new(instance, &window).pick_best_device();
}

