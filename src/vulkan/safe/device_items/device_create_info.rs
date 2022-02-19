#![allow(dead_code)]
use std::ptr;

use mira::vulkan::{VkDeviceCreateInfo, VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO};

use super::{physical_device_features::PhysicalDeviceFeaturesBuilder, device_queue_create_info::DeviceQueueCreateInfo};

pub struct DeviceCreateInfo<'a, T: Clone> {
    next: Option<T>,
    flags: Option<u32>,
    queue_create_info_count: Option<u32>,
    queue_create_infos: Option<DeviceQueueCreateInfo<T>>,
    enabled_layer_count: Option<u32>,
    enabled_layer_names: Option<Vec<&'a str>>,
    enabled_extension_count: Option<u32>,
    enabled_extension_names: Option<Vec<&'a str>>,
    enabled_features: PhysicalDeviceFeaturesBuilder,
}

impl<'a, T: Clone> DeviceCreateInfo<'a, T> {
    pub fn into_raw(&self) -> VkDeviceCreateInfo {
        return VkDeviceCreateInfo { 
            sType: VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO, 
            pNext: match &self.next {
                Some(next) => Box::into_raw(Box::new(next)) as *const _,
                None => ptr::null(),
            },
            flags: match self.flags {
                Some(flags) => flags,
                None => 0,
            }, 
            queueCreateInfoCount: match self.queue_create_info_count {
                Some(queue_create_info_count) => queue_create_info_count,
                None => 0,
            }, 
            pQueueCreateInfos: match self.queue_create_infos.clone() {
                Some(queue_create_infos) => &(queue_create_infos.into_raw()) as *const _,
                None => ptr::null_mut(),
            },
            enabledLayerCount: match self.enabled_layer_count {
                Some(enabled_layer_count) => enabled_layer_count,
                None => 0,
            },
            ppEnabledLayerNames: match &self.enabled_layer_names {
                Some(enabled_layer_names) => enabled_layer_names.as_ptr() as *const _,
                None => ptr::null(),
            },
            enabledExtensionCount: match self.enabled_extension_count {
                Some(enabled_extension_count) => enabled_extension_count,
                None => 0,
            },
            ppEnabledExtensionNames: match &self.enabled_extension_names {
                Some(enabled_extension_names) => enabled_extension_names.as_ptr() as *const _,
                None => ptr::null(),
            },
            pEnabledFeatures: &self.enabled_features.into_raw(),
        }
    }
}