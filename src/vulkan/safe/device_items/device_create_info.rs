#![allow(dead_code)]
use std::ptr;

use mira::vulkan::{VkDeviceCreateInfo, VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO};

use super::{physical_device_features::PhysicalDeviceFeaturesBuilder, device_queue_create_info::DeviceQueueCreateInfo};

/// This is our own DeviceCreateInfo struct, leaving out any fields that are redundant (at least for our purposes)
/// This simplifies the process slightly
pub struct DeviceCreateInfo<'a, T: Clone> {
    next: Option<T>,
    flags: Option<u32>,
    queue_create_info_count: Option<u32>,
    queue_create_infos: Option<Vec<DeviceQueueCreateInfo<T>>>,
    enabled_layer_count: Option<u32>,
    enabled_layer_names: Option<Vec<&'a str>>,
    enabled_extension_count: Option<u32>,
    enabled_extension_names: Option<Vec<&'a str>>,
    enabled_features: PhysicalDeviceFeaturesBuilder,
}

/// The builder for DeviceCreateInfo, everything is an Option to make this safer
pub struct DeviceCreateInfoBuilder<'a, T: Clone> {
    next: Option<T>,
    flags: Option<u32>,
    queue_create_info_count: Option<u32>,
    queue_create_infos: Option<Vec<DeviceQueueCreateInfo<T>>>,
    enabled_layer_count: Option<u32>,
    enabled_layer_names: Option<Vec<&'a str>>,
    enabled_extension_count: Option<u32>,
    enabled_extension_names: Option<Vec<&'a str>>,
    enabled_features: Option<PhysicalDeviceFeaturesBuilder>,
}

/// DeviceCreateInfo implementation 
impl<'a, T: Clone> DeviceCreateInfo<'a, T> {
    /// Puts our DeviceCreateInfo into a VkDeviceCreateInfo
    pub fn into_raw(self) -> VkDeviceCreateInfo {
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
                Some(queue_create_infos) => queue_create_infos.as_ptr() as *const _,
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
            pEnabledFeatures: &(self.enabled_features.build())
        }
    }
}

/// Builder implementation
impl<'a, T: Clone> DeviceCreateInfoBuilder<'a, T> {
    /// An empty DeviceCreateInfo which we can put information into through the functions defined below
    /// 
    /// # Examples
    /// 
    /// ```rs
    /// // This is a bad example because it will seg fault (I think)
    /// let device_create_info = DeviceCreateInfoBuilder::new()
    ///     .flags(69)
    ///     .build();
    /// ```
    pub fn new() -> Self {
        DeviceCreateInfoBuilder {
            next: None,
            flags: None,
            queue_create_info_count: None,
            queue_create_infos: None,
            enabled_layer_count: None,
            enabled_layer_names: None,
            enabled_extension_count: None,
            enabled_extension_names: None,
            enabled_features: None,
        }
    }
    pub fn next(mut self, next: T) -> Self {
        self.next = Some(next);
        self
    }
    pub fn flags(mut self, flags: u32) -> Self {
        self.flags = Some(flags);
        self
    }
    pub fn queue_create_info_count(mut self, queue_create_info_count: u32) -> Self {
        self.queue_create_info_count = Some(queue_create_info_count);
        self
    }
    pub fn queue_create_infos(mut self, queue_create_infos: Vec<DeviceQueueCreateInfo<T>>) -> Self {
        self.queue_create_infos = Some(queue_create_infos);
        self
    }
    pub fn enabled_layer_count(mut self, enabled_layer_count: u32) -> Self {
        self.enabled_layer_count = Some(enabled_layer_count);
        self
    }
    pub fn enabled_layer_names(mut self, enabled_layer_names: Vec<&'a str>) -> Self {
        self.enabled_layer_names = Some(enabled_layer_names);
        self
    }
    pub fn enabled_extension_count(mut self, enabled_extension_count: u32) -> Self {
        self.enabled_extension_count = Some(enabled_extension_count);
        self
    }
    pub fn enabled_extension_names(mut self, enabled_extension_names: Vec<&'a str>) -> Self {
        self.enabled_extension_names = Some(enabled_extension_names);
        self
    }
    pub fn enabled_features(mut self, enabled_features: PhysicalDeviceFeaturesBuilder) -> Self {
        self.enabled_features = Some(enabled_features);
        self
    }
    /// "Builds" the DeviceCreateInfo and returns it safely
    pub fn build(self) -> DeviceCreateInfo<'a, T> {
        DeviceCreateInfo {
            next: self.next,
            flags: self.flags,
            queue_create_info_count: self.queue_create_info_count,
            queue_create_infos: self.queue_create_infos,
            enabled_layer_count: self.enabled_layer_count,
            enabled_layer_names: self.enabled_layer_names,
            enabled_extension_count: self.enabled_extension_count,
            enabled_extension_names: self.enabled_extension_names,
            enabled_features: match self.enabled_features {
                Some(e) => e,
                None => PhysicalDeviceFeaturesBuilder::new(),
            },
        }
    }
}