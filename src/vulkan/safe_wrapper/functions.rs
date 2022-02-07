use std::mem::zeroed;
use std::ptr;

use mira::vulkan::{VkInstanceCreateInfo, VkAllocationCallbacks, VkInstance};

use crate::result::CinderResult;

use crate::vulkan::r#unsafe::unsafe_functions::vkCreateInstance;

pub(crate) fn create_instance(
    create_info: Option<VkInstanceCreateInfo>, 
    allocator: Option<VkAllocationCallbacks>, 
) -> CinderResult<VkInstance> {
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
        );
        let x = instance;
        CinderResult::new(result, instance)
    }
}