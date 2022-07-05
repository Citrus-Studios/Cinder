use std::ffi::CString;

use crate::vk_instancing::SafeApplicationInfo;

use super::r#unsafe::UnsafeApplicationInfo;

impl Into<UnsafeApplicationInfo> for SafeApplicationInfo {
    fn into(self) -> UnsafeApplicationInfo {
        let application_name = CString::new(self.application_name).unwrap();
        let application_name = application_name.as_ptr() as *const char;
        let engine_name = CString::new(self.engine_name).unwrap();
        let engine_name = engine_name.as_ptr() as *const char;

        unsafe {
            UnsafeApplicationInfo::new(
                application_name,
                engine_name,
                self.application_version,
                self.engine_version,
                self.api_version,
            )
        }
    }
}
