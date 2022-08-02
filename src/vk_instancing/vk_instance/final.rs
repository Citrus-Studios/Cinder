use mira::vulkan::VkInstance;

use crate::vk_instancing::SafeInstance;

pub struct Instance<'a> {
    pub normal: SafeInstance<'a>,
    pub raw: VkInstance,
}

impl<'a> Instance<'a> {
    pub fn new(normal: SafeInstance<'a>) -> Self {
        Self {
            normal: normal,
            raw: normal.into(),
        }
    }
}
