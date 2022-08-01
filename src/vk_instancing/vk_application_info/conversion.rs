use std::ffi::{CStr, CString};

use mira::vulkan::VkApplicationInfo;
use tracing::debug;

use crate::{
    structure_type::StructureType,
    vk_instancing::{ApplicationInfo, SafeApplicationInfo, UnsafeApplicationInfo},
};

impl Into<UnsafeApplicationInfo> for SafeApplicationInfo {
    fn into(self) -> UnsafeApplicationInfo {
        #[cfg(feature = "detailed-logging")]
        debug!("Converting `SafeApplicationInfo` into `UnsafeApplicationInfo`");
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

impl Into<VkApplicationInfo> for UnsafeApplicationInfo {
    fn into(self) -> VkApplicationInfo {
        #[cfg(feature = "detailed-logging")]
        debug!("Converting `UnsafeApplicationInfo` into `VkApplicationInfo`");
        VkApplicationInfo {
            sType: StructureType::ApplicationInfo as u32,
            pNext: std::ptr::null(),
            pApplicationName: self.application_name as *const i8,
            applicationVersion: self.application_version,
            pEngineName: self.engine_name as *const i8,
            engineVersion: self.engine_version,
            apiVersion: self.api_version,
        }
    }
}

impl Into<ApplicationInfo> for SafeApplicationInfo {
    fn into(self) -> ApplicationInfo {
        #[cfg(feature = "detailed-logging")]
        debug!("Converting `SafeApplicationInfo` into `ApplicationInfo`");
        ApplicationInfo::new(self)
    }
}

impl Into<SafeApplicationInfo> for UnsafeApplicationInfo {
    fn into(self) -> SafeApplicationInfo {
        #[cfg(feature = "detailed-logging")]
        debug!("Converting `UnsafeApplicationInfo` into `SafeApplicationInfo`");
        let application_name = unsafe { CStr::from_ptr(self.application_name as *const i8) };
        let application_name = application_name.to_str().unwrap();
        let engine_name = unsafe { CStr::from_ptr(self.engine_name as *const i8) };
        let engine_name = engine_name.to_str().unwrap();

        SafeApplicationInfo::new(
            application_name,
            engine_name,
            self.application_version,
            self.engine_version,
            self.api_version,
        )
    }
}

impl Into<ApplicationInfo> for UnsafeApplicationInfo {
    fn into(self) -> ApplicationInfo {
        #[cfg(feature = "detailed-logging")]
        debug!("Converting `UnsafeApplicationInfo` into `ApplicationInfo`");
        let application_info: SafeApplicationInfo = self.into();

        application_info.into()
    }
}

impl Into<VkApplicationInfo> for ApplicationInfo {
    fn into(self) -> VkApplicationInfo {
        self.raw
    }
}

impl Into<UnsafeApplicationInfo> for ApplicationInfo {
    fn into(self) -> UnsafeApplicationInfo {
        self.normal.into()
    }
}

impl Into<SafeApplicationInfo> for ApplicationInfo {
    fn into(self) -> SafeApplicationInfo {
        self.normal
    }
}
