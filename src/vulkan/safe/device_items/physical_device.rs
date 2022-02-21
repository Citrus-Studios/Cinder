use std::{sync::Arc};

use mira::{vulkan::{VkPhysicalDevice, VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU, VK_PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU, VkInstance, VkPhysicalDeviceProperties}};


use crate::{vulkan::safe::functions::{get_physical_devices, get_physical_device_properties}, match_error_codes::MatchErrorCode};

use super::{super::instance_items::instance::Instance, queue_family::QueueFamily};
#[derive(Clone)]
/// Our own physical device struct that includes a Vec of all the available physical devices, the current physical device, and the instance
pub struct PhysicalDevice {
    pub(crate) physical_devices: Vec<VkPhysicalDevice>,
    pub(crate) current_physical_device: VkPhysicalDevice,
    pub(crate) instance: VkInstance
}

/// PhysicalDevice implementation
impl PhysicalDevice {
    /// Gets the available physical devices (GPUs) and picks one to be the current
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
    /// Picks the best PhysicalDevice from the available devices
    pub fn pick_best_device(mut self) -> Self {
        self.current_physical_device = unsafe { self.rate_device_suitability(self.clone().physical_devices) };
        return self;
    }
    /// Picks the best QueueFamily
    pub fn pick_best_queue_family(&self, capabilities: u32) -> QueueFamily {
        let mut queue_family = QueueFamily::new();
        queue_family.select_queue_family(self.clone(), capabilities);
        return queue_family;
    }
    /// Rates a PhysicalDevice (GPU) based on what type it is
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
