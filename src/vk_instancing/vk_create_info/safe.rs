use crate::vk_instancing::vk_application_info::SafeApplicationInfo;

/// A Safe Wrapper for Create Info
pub struct SafeCreateInfo<'a> {
    pub application_info: &'a SafeApplicationInfo,
    pub enabled_layer_count: u32,
    pub layer_names: Vec<String>,
    pub enabled_extension_count: u32,
    pub enabled_extension_names: Vec<String>,
}

impl<'a> SafeCreateInfo<'a> {
    pub fn new() -> Self {
        todo!();
    }
    /// Creates a new Safe Create Info
    ///
    /// ```rust
    /// use cinder::vulkan_instance::{create_info::SafeCreateInfo, application_info::SafeApplicationInfo};
    ///
    /// let application_info = SafeApplicationInfo::new_strings("Application Name", "Engine Name", "1.0.0", "1.0.0", "0.1.0.0");
    /// let create_info = SafeCreateInfo::new_from(&application_info);
    /// ```
    pub fn new_from(
        application_info: &'a SafeApplicationInfo,
        enabled_layer_count: u32,
        layer_names: Vec<String>,
        enabled_extension_count: u32,
        enabled_extension_names: Vec<String>,
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
