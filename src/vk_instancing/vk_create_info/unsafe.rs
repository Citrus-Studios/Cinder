use tracing::debug;

use crate::vk_instancing::SafeApplicationInfo;

/// Unsafe version of UnsafeCreateInfo
pub struct UnsafeCreateInfo {
    pub application_info: *const SafeApplicationInfo,
    pub enabled_layer_count: u32,
    pub layer_names: *const *const char,
    pub enabled_extension_count: u32,
    pub enabled_extension_names: *const *const char,
}

impl UnsafeCreateInfo {
    pub unsafe fn new(
        application_info: *const SafeApplicationInfo,
        enabled_layer_count: u32,
        layer_names: *const *const char,
        enabled_extension_count: u32,
        enabled_extension_names: *const *const char,
    ) -> Self {
        debug!(
            "Created `UnsafeCreateInfo` with arguments `{:?}`, `{}`, `{:?}`, `{}`, `{:?}`",
            application_info,
            enabled_layer_count,
            layer_names,
            enabled_extension_count,
            enabled_extension_names
        );
        Self {
            application_info,
            enabled_layer_count,
            layer_names,
            enabled_extension_count,
            enabled_extension_names,
        }
    }
}
