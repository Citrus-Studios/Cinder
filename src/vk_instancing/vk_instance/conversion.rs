use mira::vulkan::VkInstance;
use tracing::debug;

use crate::vk_instancing::SafeInstance;

use super::r#unsafe::UnsafeInstance;

impl<'a> Into<UnsafeInstance<'a>> for SafeInstance<'a> {
    fn into(self) -> UnsafeInstance<'a> {
        #[cfg(feature = "detailed-logging")]
        debug!("Converting `SafeInstance` into `UnsafeInstance`");
        UnsafeInstance {
            create_info: self.create_info,
            allocator: std::ptr::null(),
        }
    }
}
