use std::mem::zeroed;
use std::ptr;

use mira::mem::zeroed_vec;
use mira::vulkan::{VkPhysicalDevice, VK_SUCCESS, VkPhysicalDeviceProperties, VkQueueFamilyProperties, VkSurfaceKHR, VkDevice};
use mira::vulkan::{VkInstanceCreateInfo, VkAllocationCallbacks, VkInstance};

use crate::vulkan::r#unsafe::unsafe_functions::*;

use super::device_items::device_create_info::DeviceCreateInfo;
use super::device_items::physical_device::PhysicalDevice;

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

pub(crate) fn get_physical_device_properties(
    physical_device: VkPhysicalDevice,
    instance: VkInstance,
) -> VkPhysicalDeviceProperties {
    let mut properties = unsafe { zeroed::<VkPhysicalDeviceProperties>() };
    vkGetPhysicalDeviceProperties(physical_device, &mut properties, Some(instance));
    return properties;
}

pub(crate) fn get_physical_device_queue_family_properties(
    physical_device: VkPhysicalDevice,
    instance: VkInstance,
) -> Vec<VkQueueFamilyProperties> {
    let mut amount = 0u32;
    vkGetPhysicalDeviceQueueFamilyProperties(physical_device, &mut amount, ptr::null_mut(), Some(instance));
    
    let mut properties = unsafe { zeroed_vec::<VkQueueFamilyProperties>(amount as usize) };
    vkGetPhysicalDeviceQueueFamilyProperties(physical_device, &mut amount, properties.as_mut_ptr(), Some(instance));
    
    return properties;
}

#[test] 
pub fn optional_surface_test() {
    use crate::vulkan::safe::instance_items::instance::InstanceBuilder;

    let instance = InstanceBuilder::new()
        .application_name("Triangle")
        .build();
    let physical_devices = PhysicalDevice::new(instance.clone()).pick_best_device();
    let instance_clone = *instance.clone();
    let properties = get_physical_device_properties(physical_devices.current_physical_device, instance_clone.instance);
    println!("{:?}", properties);
}

pub(crate) fn physical_device_surface_support(
    physical_device: VkPhysicalDevice,
    queue_family_index: u32,
    surface: Option<VkSurfaceKHR>,
    instance: VkInstance,
) -> bool {
    let mut supported = 0u32;
    let result = vkGetPhysicalDeviceSurfaceSupportKHR(physical_device, queue_family_index, match surface {
        Some(surface) => surface,
        None => ptr::null_mut(),
    }, &mut supported, Some(instance));
    match result {
        VK_SUCCESS => {},
        _ => {
            panic!("Failed to get physical device surface support");
        }
    }
    return match supported {
        0 => false,
        _ => true,
    };
}

pub(crate) fn create_device<T: Clone>(
    physical_device: PhysicalDevice,
    create_info: Option<DeviceCreateInfo<T>>,
    allocator: Option<VkAllocationCallbacks>,
) -> Result<VkDevice, i32> {
    let mut device: VkDevice = unsafe { zeroed() };
    let result = vkCreateDevice(
        physical_device.current_physical_device, 
        match create_info {
            Some(createinfo) => &createinfo.into_raw() as *const _,
            None => ptr::null(),
        }, 
        match allocator {
            Some(mut allocator) => &mut allocator as *mut _,
            None => ptr::null_mut(),
        }, 
        &mut device, 
        Some(physical_device.instance)
    );
    if result != 0 {
        return Err(result);
    }
    return Ok(device);
}