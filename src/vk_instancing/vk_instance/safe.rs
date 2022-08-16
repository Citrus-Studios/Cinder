use crate::vk_instancing::CreateInfo;

/// A Safe Wrapper for a Vulkan instance to try and prevent many, many headaches of unsafe
#[derive(Clone)]
pub struct SafeInstance<'a> {
    pub create_info: CreateInfo<'a>,
    pub allocator: Option<()>,
}

impl<'a> SafeInstance<'a> {
    pub fn new(create_info: CreateInfo<'a>) -> Self {
        Self {
            create_info,
            allocator: None,
        }
    }
}
