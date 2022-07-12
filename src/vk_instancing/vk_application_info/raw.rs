use mira::vulkan::VkApplicationInfo;

use crate::vk_instancing::SafeApplicationInfo;

use super::r#unsafe::UnsafeApplicationInfo;

pub struct ApplicationInfo {
    raw: VkApplicationInfo,
}

impl ApplicationInfo {
    pub fn new(application_info: SafeApplicationInfo) -> Self {
        Self {
            raw: <SafeApplicationInfo as Into<UnsafeApplicationInfo>>::into(application_info)
                .into(),
        }
    }
}
