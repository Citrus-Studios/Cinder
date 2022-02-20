use std::{sync::Arc};

use mira::{vulkan::{VkPhysicalDevice, VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU, VK_PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU, VkInstance, VkPhysicalDeviceProperties}};


use crate::{vulkan::safe::functions::{get_physical_devices, get_physical_device_properties}, match_error_codes::MatchErrorCode};

use super::{super::instance_items::instance::Instance, queue_family::QueueFamily};
#[derive(Clone)]
pub struct PhysicalDevice {
    pub(crate) physical_devices: Vec<VkPhysicalDevice>,
    pub(crate) current_physical_device: VkPhysicalDevice,
    pub(crate) instance: VkInstance
}

impl PhysicalDevice {
    pub fn new(instance: Arc<Instance>) -> Self {
        let devices = get_physical_devices((*instance.clone()).instance);
        match devices {
            Ok(devices) => PhysicalDevice {
                physical_devices: devices.clone(),
                current_physical_device: devices[0],
                instance: (*instance.clone()).instance
            },
            Err(error) => panic!("Failed to get physical devices: {}", error.match_error_code()),
        }
    }
    pub fn pick_best_device(mut self) -> Self {
        self.current_physical_device = unsafe { self.rate_device_suitability(self.clone().physical_devices) };
        return self;
    }
    pub fn pick_best_queue_family(&self, capiabilities: u32) -> QueueFamily {
        let mut queue_family = QueueFamily::new();
        queue_family.select_queue_family(self.clone(), capiabilities);
        return queue_family;
    }
    pub(crate) unsafe fn rate_device_suitability(&self, devices: Vec<VkPhysicalDevice>) -> VkPhysicalDevice {
        let mut selected_device: VkPhysicalDevice = std::mem::zeroed();
        let gpu_range = VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU ..= VK_PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU;
    
        for device in devices.iter() {
            let device_properties: VkPhysicalDeviceProperties = std::mem::zeroed();
            get_physical_device_properties(*device, self.instance);
    
            if gpu_range.contains(&device_properties.deviceType) {
                selected_device = *device;
                break;
            }
        }
        // fallback if fails to find a suitable device
        if selected_device == std::mem::zeroed() {
            selected_device = devices[0];
        }

        return selected_device;
    }
} 
