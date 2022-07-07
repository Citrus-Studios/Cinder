//! Safe Wrapper for Vulkan Create Info
//!
//! ```rust
//! use cinder::vk_instancing::{SafeApplicationInfo, SafeCreateInfo};
//!
//! let application_info = SafeApplicationInfo::new_strings("Application Name", "Engine Name", "1.0.0", "1.0.0", "0.1.0.0");
//! let create_info = SafeCreateInfo::new_from(&application_info);
//! ```

use tracing::debug;

use crate::vk_instancing::SafeApplicationInfo;

/// A Safe Wrapper for Create Info
pub struct SafeCreateInfo<'a> {
    pub application_info: &'a SafeApplicationInfo,
    pub enabled_layer_count: u32,
    pub layer_names: Vec<String>,
    pub enabled_extension_count: u32,
    pub enabled_extension_names: Vec<String>,
}

impl<'a> SafeCreateInfo<'a> {
    /// Creates a new Safe Create Info
    ///
    /// ```rust
    /// use cinder::vk_instancing::{SafeCreateInfo, SafeApplicationInfo};
    ///
    /// let application_info = SafeApplicationInfo::new_strings("Application Name", "Engine Name", "1.0.0", "1.0.0", "0.1.0.0");
    /// let create_info = SafeCreateInfo::auto_new(&application_info);
    /// ```
    pub fn auto_new(application_info: &'a SafeApplicationInfo) -> Self {
        todo!();
    }
    // TODO: Finish the Doc comment for this function
    /// Creates a new Safe Create Info
    ///
    /// ```rust
    /// use cinder::vk_instancing::{SafeCreateInfo, SafeApplicationInfo};
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
        #[cfg(feature = "heavy-logging")]
        debug!(
            "Creating `SafeCreateInfo` using `new_from` function with arguments `{:?}`, `{}`, `{:?}`, `{}`, `{:?}`",
            application_info,
            enabled_layer_count,
            layer_names,
            enabled_extension_count,
            enabled_extension_names
        );
        #[cfg(feature = "medium-logging")]
        debug!("Creating `SafeCreateInfo` using `new_from` function");
        Self {
            application_info,
            enabled_layer_count,
            layer_names,
            enabled_extension_count,
            enabled_extension_names,
        }
    }
}
