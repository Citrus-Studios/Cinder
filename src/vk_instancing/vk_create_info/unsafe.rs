use crate::vk_instancing::vk_application_info::safe::SafeApplicationInfo;

/// Unsafe version of CreateInfo
pub struct CreateInfo {
    pub application_info: *const SafeApplicationInfo,
    pub enabled_layer_count: u32,
    pub layer_names: *const *const char,
    pub enabled_extension_count: u32,
    pub enabled_extension_names: *const *const char,
}

impl CreateInfo {
    pub unsafe fn new(
        application_info: *const SafeApplicationInfo,
        enabled_layer_count: u32,
        layer_names: *const *const char,
        enabled_extension_count: u32,
        enabled_extension_names: *const *const char,
    ) -> Self {
        Self {
            application_info,
            enabled_layer_count,
            layer_names,
            enabled_extension_count,
            enabled_extension_names,
        }
    }
}
