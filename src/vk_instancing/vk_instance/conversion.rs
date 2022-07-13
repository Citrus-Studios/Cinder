use crate::vk_instancing::SafeInstance;

use super::r#unsafe::UnsafeInstance;

impl<'a> Into<UnsafeInstance<'a>> for SafeInstance<'a> {
    fn into(self) -> UnsafeInstance<'a> {
        UnsafeInstance {
            create_info: self.create_info,
            allocator: std::ptr::null(),
        }
    }
}
