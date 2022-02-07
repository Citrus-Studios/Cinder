use std::mem::zeroed;
use std::ptr;

use mira::mem::zeroed_vec;
use mira::vulkan::{VkPhysicalDevice, VK_SUCCESS};
use mira::vulkan::{VkInstanceCreateInfo, VkAllocationCallbacks, VkInstance};

use crate::vulkan::r#unsafe::unsafe_functions::vkCreateInstance;
use crate::vulkan::r#unsafe::unsafe_functions::vkEnumeratePhysicalDevices;

pub(crate) fn create_instance(
    create_info: Option<VkInstanceCreateInfo>, 
    allocator: Option<VkAllocationCallbacks>, 
) -> Result<VkInstance, i32> {
    unsafe {
        let mut instance: VkInstance = zeroed();
        let result = vkCreateInstance(
            match create_info {
                Some(create_info) => &create_info as *const _,
                None => ptr::null(),
            },
            match allocator {
                Some(mut allocator) => &mut allocator as *mut _,
                None => ptr::null_mut(),
            },
            &mut instance,
            None
        );
        match result {
            VK_SUCCESS => Ok(instance),
            _ => Err(result),
        }
    }
}

pub(crate) fn get_physical_devices(
    instance: VkInstance,
) -> Result<Vec<VkPhysicalDevice>, i32> {
    let mut amount = 0u32;
    match vkEnumeratePhysicalDevices(instance, &mut amount, ptr::null_mut(), Some(instance)) {
        VK_SUCCESS => {},
        error => {
            return Err(error);
        }
    }

    let mut devices = unsafe { zeroed_vec::<VkPhysicalDevice>(amount as usize) };
    match vkEnumeratePhysicalDevices(instance, &mut amount, devices.as_mut_ptr(), Some(instance)) {
        VK_SUCCESS => {},
        error => {
            return Err(error);
        }
    }
    Ok(devices)
}