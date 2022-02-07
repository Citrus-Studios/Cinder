use std::{sync::Arc};

use mira::vulkan::{VkPhysicalDevice};


use crate::{vulkan::safe_wrapper::functions::get_physical_devices, match_error_codes::MatchErrorCode};

use super::instance::Instance;

pub struct PhysicalDevice {
    physical_device: Vec<VkPhysicalDevice>,
}

impl PhysicalDevice {
    pub fn new(instance: Arc<Instance>) -> Self {
        let devices = get_physical_devices((*instance.clone()).instance);
        match devices {
            Ok(devices) => PhysicalDevice {
                physical_device: devices,
            },
            Err(error) => panic!("Failed to get physical devices: {}", error.match_error_code()),
        }
    }
}