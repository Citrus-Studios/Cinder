use std::{
    mem::zeroed,
    ptr::{null, null_mut},
};

use mira::vulkan::VkInstance;

use crate::match_error_codes::MatchErrorCode;
#[allow(unused_imports)]
use crate::unsafe_functions::*;
use crate::vk_instancing::CreateInfo;

/// # Taken from https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_MAKE_VERSION.html<br/>
/// Creates a Vulkan version from 3 numbers
#[deprecated(note = "Deprecated in vulkan please use `make_api_version` instead")]
pub fn make_version(major: u8, minor: u8, patch: u8) -> u32 {
    (major as u32) << 22 | (minor as u32) << 12 | patch as u32
}
/// # Taken from https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_MAKE_API_VERSION.html<br/>
/// Creates a Vulkan version from 4 numbers
pub fn make_api_version(variant: u8, major: u8, minor: u8, patch: u8) -> u32 {
    (variant as u32) << 29 | (major as u32) << 22 | (minor as u32) << 12 | (patch as u32)
}

pub(crate) fn create_instance(
    create_info: Option<CreateInfo>,
    allocator: Option<()>,
) -> Result<VkInstance, String> {
    unsafe {
        let mut instance: VkInstance = zeroed();
        let result = vkCreateInstance(
            match create_info {
                Some(create_info) => &(create_info.into()),
                None => null(),
            },
            match allocator {
                Some(allocator) => null_mut(),
                None => null_mut(),
            },
            &mut instance,
            None,
        );
        match result {
            VK_SUCCESS => Ok(instance),
            _ => Err(result.match_vulkan_error_code().to_string()),
        }
    }
}
