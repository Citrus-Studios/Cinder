use mira::vulkan::{VkSurfaceKHR};

use super::{physical_device::PhysicalDevice, super::functions::{get_physical_device_queue_family_properties, physical_device_surface_support}};

pub struct QueueFamily {
    pub selected_queuefamily: Option<u32>,
}

impl QueueFamily {
    pub fn new() -> Self {
        Self {
            selected_queuefamily: None,
        }
    }

    pub fn select_queue_family(&mut self, device: PhysicalDevice, capabilities: u32, surface: VkSurfaceKHR) -> Self {
        let queue_family_properties = get_physical_device_queue_family_properties(device.current_physical_device, device.instance);

        for x in queue_family_properties.into_iter().enumerate() {
            let support = physical_device_surface_support(device.current_physical_device, x.0 as u32, surface, device.instance);
            if (x.1.queueFlags & capabilities) == capabilities 
            && support != false {
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