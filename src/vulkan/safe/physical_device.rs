use std::{sync::Arc};

use mira::{vulkan::{VkPhysicalDevice, VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU, VK_PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU, VkInstance, VkPhysicalDeviceProperties}};


use crate::{vulkan::safe::functions::{get_physical_devices, get_physical_device_properties}, match_error_codes::MatchErrorCode};

use super::instance::Instance;
#[derive(Clone)]
pub struct PhysicalDevice {
    physical_devices: Vec<VkPhysicalDevice>,
    current_physical_device: VkPhysicalDevice,
    instance: VkInstance
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
        self.current_physical_device = unsafe { self.clone().rate_device_suitability(self.clone().physical_devices) };
        return self;
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

        return selected_device;
    }
} 
