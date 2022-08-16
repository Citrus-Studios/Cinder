use mira::vulkan::VkInstance;

use crate::{functions::create_instance, vk_instancing::SafeInstance};

pub struct Instance<'a> {
    pub normal: SafeInstance<'a>,
    pub raw: VkInstance,
}

impl<'a> Instance<'a> {
    pub fn new(normal: SafeInstance<'a>) -> Self {
        Self {
            normal: normal.clone(),
            raw: create_instance(Some(normal.create_info), None)
                .unwrap()
                .into(),
        }
    }
}
