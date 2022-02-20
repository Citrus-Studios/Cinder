use super::{physical_device::PhysicalDevice, super::functions::{get_physical_device_queue_family_properties, physical_device_surface_support}};

#[test]
pub fn test_get_physical_device_queue_family_properties() {
    use crate::vulkan::safe::{instance_items::instance::InstanceBuilder, device_items::physical_device::PhysicalDevice};
    let instance = InstanceBuilder::new()
        .application_name("Triangle")
        .build();
    let physical_device = PhysicalDevice::new(instance);
    let _queue_family_properties = get_physical_device_queue_family_properties(physical_device.current_physical_device, physical_device.instance);
}

pub struct QueueFamily {
    pub selected_queuefamily: Option<u32>,
}

impl QueueFamily {
    pub fn new() -> Self {
        Self {
            selected_queuefamily: None,
        }
    }

    pub fn select_queue_family(&mut self, device: PhysicalDevice, capabilities: u32) -> Self {
        let queue_family_properties = get_physical_device_queue_family_properties(device.current_physical_device, device.instance);

        for x in queue_family_properties.into_iter().enumerate() {
            if (x.1.queueFlags & capabilities) == capabilities 
            && physical_device_surface_support(device.current_physical_device, x.0 as u32, None, device.instance) != false {
                self.selected_queuefamily = Some(x.0 as u32);
                break;
            }
        }

        panic!("Physical Device not found");
    }
    pub fn get_selected_queue_family(&self) -> u32 {
        self.selected_queuefamily.unwrap()
    }
}