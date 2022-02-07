use std::{ptr, sync::Arc};

use mira::vulkan::{VkPhysicalDevice};


use super::instance::Instance;

pub struct PhysicalDevice {
    physical_device: VkPhysicalDevice,
    device_count: u32,
}

impl PhysicalDevice {
    pub fn new(instance: Arc<Instance>) -> Self {
        println!("code ran?");
        let new_instance = instance.clone();
        println!("code ran?");
        let device_count = 0 as *mut u32;
        let mut devices: VkPhysicalDevice = unsafe { std::mem::zeroed() };
        println!("code ran?");
        let device_result = vkEnumeratePhysicalDevices(new_instance.instance, device_count, &mut devices as *mut VkPhysicalDevice);
        println!("code ran?");
        if device_count as i32 == 0 {
            panic!("No physical devices found");
        }
        
        println!("{:?}", devices);
        unsafe { PhysicalDevice { 
            physical_device: ptr::null_mut(),
            device_count:  *device_count,
        }}
    }
}