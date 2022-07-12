use mira::vulkan::VkApplicationInfo;

use crate::vk_instancing::SafeApplicationInfo;

use super::r#unsafe::UnsafeApplicationInfo;

pub struct ApplicationInfo {
    raw: VkApplicationInfo,
    normal: SafeApplicationInfo,
}

impl ApplicationInfo {
    pub fn new(application_info: SafeApplicationInfo) -> Self {
        Self {
            raw: <SafeApplicationInfo as Into<UnsafeApplicationInfo>>::into(
                application_info.clone(),
            )
            .into(),
            normal: application_info,
        }
    }
    pub fn rebuild(&mut self) {
        self.raw =
            <SafeApplicationInfo as Into<UnsafeApplicationInfo>>::into(self.normal.clone()).into();
    }
}
