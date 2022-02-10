#[test]
fn triangle() {
    use crate::vulkan::safe_wrapper::{instance::InstanceBuilder, physical_device::PhysicalDevice};

    let instance = InstanceBuilder::new()
        .application_name("Cinder")
        .engine_name("None")
        .build();

    let _physical_device = PhysicalDevice::new(instance).pick_best_device();
}

