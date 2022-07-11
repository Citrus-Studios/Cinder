use std::ffi::CString;

use tracing::debug;

use crate::vk_instancing::{SafeApplicationInfo, SafeCreateInfo, UnsafeCreateInfo};

impl<'a> Into<UnsafeCreateInfo> for SafeCreateInfo<'a> {
    fn into(self) -> UnsafeCreateInfo {
        #[cfg(feature = "heavy-logging")]
        debug!("Converting `SafeCreateInfo` into `UnsafeCreateInfo`");
        let layer_names = self
            .layer_names
            .iter()
            .map(|x| {
                let string = CString::new(x.clone()).unwrap();
                let string = string.as_ptr() as *const char;
                string
            })
            .collect::<Vec<_>>()
            .into_raw_parts()
            .0 as *const *const char;
        let extension_names = self
            .enabled_extension_names
            .iter()
            .map(|x| {
                let string = CString::new(x.clone()).unwrap();
                let string = string.as_ptr() as *const char;
                string
            })
            .collect::<Vec<_>>()
            .into_raw_parts()
            .0 as *const *const char;

        UnsafeCreateInfo {
            application_info: self.application_info as *const SafeApplicationInfo,
            enabled_layer_count: self.enabled_layer_count,
            layer_names: layer_names,
            enabled_extension_count: self.enabled_extension_count,
            enabled_extension_names: extension_names,
        }
    }
}
