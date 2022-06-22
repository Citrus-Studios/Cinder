use crate::functions::make_api_version;

/// Safe Wrapper for Application Info to prevent unsafe headaches.
pub struct SafeApplicationInfo {
    pub application_name: String,
    pub engine_name: String,
    pub application_version: u32,
    pub engine_version: u32,
    pub api_version: u32,
}

impl SafeApplicationInfo {
    /// Creates a new SafeApplicationInfo from an application_name, engine_name, application_version, engine_version and api_version
    ///
    /// ```rust
    /// use cinder::vulkan_instance::application_info::SafeApplicationInfo;
    /// use cinder::functions::make_api_version;
    ///
    /// let vulkan_version = make_api_version(0, 1, 0, 0);
    /// let engine_version = make_api_version(0, 1, 0, 0);
    /// let application_version = make_api_version(0, 1, 0, 0);
    /// let info = SafeApplicationInfo::new("Application Name", "Engine Name", application_version, engine_version, vulkan_version);
    /// ```
    pub fn new(
        application_name: &str,
        engine_name: &str,
        application_version: u32,
        engine_version: u32,
        api_version: u32,
    ) -> Self {
        Self {
            application_name: application_name.to_string(),
            engine_name: engine_name.to_string(),
            application_version,
            engine_version,
            api_version,
        }
    }
    /// Creates a new SafeApplicationInfo from an application_name, engine_name, application_version, engine_version and api_version
    ///
    /// ```rust
    /// use cinder::vulkan_instance::application_info::SafeApplicationInfo;
    /// use cinder::functions::make_api_version;
    ///
    /// let info = SafeApplicationInfo::new_strings("Application Name", "Engine Name", "1.0.0", "1.0.0", "0.1.0.0");
    /// ```
    pub fn new_strings(
        application_name: &str,
        engine_name: &str,
        application_version: &str,
        engine_version: &str,
        api_version: &str,
    ) -> Self {
        let application_version = application_version
            .split(".")
            .map(|x| x.parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
        let application_version = make_api_version(
            0,
            application_version[0],
            application_version[1],
            application_version[2],
        );
        let engine_version = engine_version
            .split(".")
            .map(|x| x.parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
        let engine_version =
            make_api_version(0, engine_version[0], engine_version[1], engine_version[2]);
        let api_version = api_version
            .split(".")
            .map(|x| x.parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
        let api_version = make_api_version(
            api_version[0],
            api_version[1],
            api_version[2],
            api_version[3],
        );
        Self {
            application_name: application_name.to_string(),
            engine_name: engine_name.to_string(),
            application_version,
            engine_version,
            api_version,
        }
    }
}
