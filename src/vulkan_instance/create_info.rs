use super::application_info::SafeApplicationInfo;

/// A Safe Wrapper for Create Info
pub struct SafeCreateInfo<'a> {
    pub application_info: &'a SafeApplicationInfo,
}

impl<'a> SafeCreateInfo<'a> {
    /// Creates a new Safe Create Info
    ///
    /// ```rust
    /// use cinder::vulkan_instance::{create_info::SafeCreateInfo, application_info::SafeApplicationInfo};
    ///
    /// let application_info = SafeApplicationInfo::new_strings("Application Name", "Engine Name", "1.0.0", "1.0.0", "0.1.0.0");
    /// let create_info = SafeCreateInfo::new(&application_info);
    /// ```
    pub fn new(application_info: &'a SafeApplicationInfo) -> Self {
        Self { application_info }
    }
}
