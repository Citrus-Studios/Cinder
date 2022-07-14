use crate::vk_instancing::CreateInfo;

pub struct UnsafeInstance<'a> {
    pub create_info: CreateInfo<'a>,
    pub allocator: *const (),
}

impl<'a> UnsafeInstance<'a> {
    pub unsafe fn new(create_info: CreateInfo<'a>) -> Self {
        Self {
            create_info,
            allocator: std::ptr::null(),
        }
    }
}
