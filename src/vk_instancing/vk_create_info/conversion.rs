use std::ffi::CString;

use mira::vulkan::{VkApplicationInfo, VkInstanceCreateInfo};
use tracing::debug;

use crate::{
    structure_type::StructureType,
    vk_instancing::{
        CreateInfo, SafeApplicationInfo, SafeCreateInfo, UnsafeApplicationInfo, UnsafeCreateInfo,
    },
};

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
            application_info: &<UnsafeApplicationInfo as Into<VkApplicationInfo>>::into(
                <SafeApplicationInfo as Into<UnsafeApplicationInfo>>::into(
                    self.application_info.clone().normal,
                ),
            ),
            enabled_layer_count: self.enabled_layer_count,
            layer_names: layer_names,
            enabled_extension_count: self.enabled_extension_count,
            enabled_extension_names: extension_names,
        }
    }
}

impl Into<VkInstanceCreateInfo> for UnsafeCreateInfo {
    fn into(self) -> VkInstanceCreateInfo {
        #[cfg(feature = "detailed-logging")]
        debug!("Converting `UnsafeCreateInfo` into `VkInstanceCreateInfo`");
        VkInstanceCreateInfo {
            sType: StructureType::InstanceCreateInfo as u32,
            pNext: std::ptr::null(),
            flags: 0,
            pApplicationInfo: self.application_info,
            enabledLayerCount: self.enabled_layer_count,
            ppEnabledLayerNames: self.layer_names as *const *const i8,
            enabledExtensionCount: self.enabled_extension_count,
            ppEnabledExtensionNames: self.enabled_extension_names as *const *const i8,
        }
    }
}

impl<'a> Into<VkInstanceCreateInfo> for CreateInfo<'a> {
    fn into(self) -> VkInstanceCreateInfo {
        #[cfg(feature = "detailed-logging")]
        debug!("Converting `CreateInfo` into `VkInstanceCreateInfo`");
        self.raw
    }
}

impl<'a> Into<CreateInfo<'a>> for UnsafeCreateInfo {
    fn into(self) -> CreateInfo<'a> {
        let safe: SafeCreateInfo = self.into();
        safe.into()
    }
}

impl<'a> Into<CreateInfo<'a>> for SafeCreateInfo<'a> {
    fn into(self) -> CreateInfo<'a> {
        CreateInfo::new(self)
    }
}
