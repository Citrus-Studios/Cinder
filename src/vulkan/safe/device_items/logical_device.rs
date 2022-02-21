#![allow(dead_code)]
use mira::vulkan::{VkDevice, VkAllocationCallbacks};

use crate::vulkan::safe::{functions::create_device, };

use super::{physical_device::PhysicalDevice, device_create_info::DeviceCreateInfo};

#[test]
fn create_device_test() {
    use crate::vulkan::safe::{instance_items::instance::InstanceBuilder, device_items::device_create_info::DeviceCreateInfoBuilder};

    let instance = InstanceBuilder::new().build();
    let physical_device = PhysicalDevice::new(instance);
    let device_create_info = DeviceCreateInfoBuilder::<()>::new().build();
    let _device = LogicalDevice::new(physical_device, device_create_info, None);
}

/// LogicalDevice is a software representation of the GPU
pub struct LogicalDevice {
    device: VkDevice,
}

/// LogicalDevice implementation
impl LogicalDevice {
    /// Creates a new LogicalDevice from the PhysicalDevice
    /// 
    /// # Example
    /// 
    /// ```rs
    /// let logical_device = LogicalDevice::new(physical_device, create_info, allocator);
    /// ```
    pub fn new<T: Clone>(physical_device: PhysicalDevice, create_info: DeviceCreateInfo<T>, allocator: Option<VkAllocationCallbacks>) -> Self {
        LogicalDevice {
            device: create_device(physical_device.clone(), Some(create_info), allocator).unwrap(),
        }
    }
}