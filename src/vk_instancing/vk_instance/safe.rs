use crate::vk_instancing::CreateInfo;

/// A Safe Wrapper for a Vulkan instance to try and prevent many, many headaches of unsafe
pub struct SafeInstance<'a> {
    create_info: CreateInfo<'a>,
    allocator: Option<()>,
}

impl<'a> SafeInstance<'a> {
    pub fn new(create_info: CreateInfo<'a>) -> Self {
        Self {
            create_info,
            allocator: None,
        }
    }
}
