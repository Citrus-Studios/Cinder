//! Safe Wrapper for Vulkan Application Info which is used for Vulkan Instances
//!
//! # Example
//! ```rust
//! use cinder::vk_instancing::SafeApplicationInfo;
//! use cinder::functions::make_api_version;
//!
//! let vulkan_version = make_api_version(0, 1, 0, 0);
//! let engine_version = make_api_version(0, 1, 0, 0);
//! let application_version = make_api_version(0, 1, 0, 0);
//! let info = SafeApplicationInfo::new("Application Name", "Engine Name", application_version, engine_version, vulkan_version);
//! ```

use tracing::debug;

use crate::functions::make_api_version;

/// Safe Wrapper for Application Info to prevent unsafe headaches.
#[derive(Debug)]
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
    /// Please use `make_api_version` from `cinder::functions` to create the `u32s` for the application_version, engine_version, and api_version arguments
    ///
    /// ```rust
    /// use cinder::vk_instancing::SafeApplicationInfo;
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
        debug!(
            "Creating `SafeApplicationInfo` with arguments `{}`, `{}`, `{}`, `{}`, `{}`",
            application_name, engine_name, application_version, engine_version, api_version
        );
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
    /// Will panic if the application_name, engine_name and api_version are not in the correct format.
    ///
    /// Application and Engine version being in the format of `X.X.X`
    ///
    /// API version in the format of `X.X.X.X`
    /// ```rust
    /// use cinder::vk_instancing::SafeApplicationInfo;
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
        const PARSE_ERROR: &'static str =
            "You can not use negative version numbers or numbers above 255";

        // Split the application_version and make sure it contains 3 elements
        let application_version = application_version
            .split(".")
            .map(|x| x.parse::<u8>().expect(PARSE_ERROR))
            .collect::<Vec<u8>>();
        assert!(3 == application_version.len());
        let application_version = make_api_version(
            0,
            application_version[0],
            application_version[1],
            application_version[2],
        );

        // Split the engine_version and make sure it contains 3 elements
        let engine_version = engine_version
            .split(".")
            .map(|x| x.parse::<u8>().expect(PARSE_ERROR))
            .collect::<Vec<u8>>();
        assert!(3 == engine_version.len());
        let engine_version =
            make_api_version(0, engine_version[0], engine_version[1], engine_version[2]);

        // Split the api_version and make sure it contains 4 elements
        let api_version = api_version
            .split(".")
            .map(|x| x.parse::<u8>().expect(PARSE_ERROR))
            .collect::<Vec<u8>>();
        assert!(4 == api_version.len());
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
