use mira::vulkan::VkInstanceCreateInfo;

use crate::vk_instancing::{SafeCreateInfo, UnsafeCreateInfo};

#[derive(Clone)]
pub struct CreateInfo<'a> {
    pub raw: VkInstanceCreateInfo,
    pub normal: SafeCreateInfo<'a>,
}

impl<'a> CreateInfo<'a> {
    pub fn new(create_info: SafeCreateInfo<'a>) -> Self {
        Self {
            raw: <SafeCreateInfo as Into<UnsafeCreateInfo>>::into(create_info.clone()).into(),
            normal: create_info,
        }
    }
    pub fn rebuild(&mut self) {
        self.raw = <SafeCreateInfo as Into<UnsafeCreateInfo>>::into(self.normal.clone()).into();
    }
}
