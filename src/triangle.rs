use crate::vulkan::safe_wrapper::{instance::InstanceBuilder, physical_device::PhysicalDevice};


#[test]
fn triangle() {
    let instance = InstanceBuilder::new()
        .application_name("Triangle")
        .build();

    let physical_device = PhysicalDevice::new(instance);
}

