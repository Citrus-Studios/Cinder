//! Builder for Application Info that creates a `SafeApplicationInfo`
//!
//! ```rust
//! use cinder::vk_instance::ApplicationInfoBuilder;
//!
//! let application_info = ApplicationInfoBuilder::new().
//! ```

use crate::{functions::make_api_version, helper::DefaultingValue};

pub struct ApplicationInfoBuilder {
    application_name: DefaultingValue<String>,
    engine_name: DefaultingValue<String>,
    application_version: DefaultingValue<u32>,
    engine_version: DefaultingValue<u32>,
    api_version: DefaultingValue<u32>,
}

impl ApplicationInfoBuilder {
    /// Creates a new `ApplicationInfoBuilder`
    ///
    /// With the defualt values of
    /// application_name: "Cinder Application"
    /// engine_name: "Cinder Engine"
    /// application_version: "1.0.0"
    /// engine_version: "1.0.0"
    /// api_version: "0.1.0.0"
    pub fn new() -> Self {
        Self {
            application_name: DefaultingValue::Default(String::from("Cinder Application")),
            engine_name: DefaultingValue::Default(String::from("Cinder Engine")),
            application_version: DefaultingValue::Default(make_api_version(0, 1, 0, 0)),
            engine_version: DefaultingValue::Default(make_api_version(0, 1, 0, 0)),
            api_version: DefaultingValue::Default(make_api_version(0, 1, 0, 0)),
        }
    }
    /// Changes the Application Name
    pub fn with_application_name(mut self, name: &str) -> Self {
        self.application_name = DefaultingValue::Unique(String::from(name));
        self
    }
    /// Changes the Engine Name
    pub fn with_engine_name(mut self, engine_name: &str) -> Self {
        self.engine_name = DefaultingValue::Unique(String::from(engine_name));
        self
    }
    /// Changes the Application Version using a u32
    pub fn with_application_version_u32(mut self, version: u32) -> Self {
        self.application_version = DefaultingValue::Unique(version);
        self
    }
    /// Changes the Application Version using a str
    /// with the format of `X.X.X`
    /// Will panic if the format is supplied correctly
    pub fn with_application_version_str(mut self, version: &str) -> Self {
        // Split the application_version and make sure it contains 3 or more elements
        let version = version
            .split(".")
            .map(|x| x.parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
        assert!(3 == version.len());
        let version = make_api_version(0, version[0], version[1], version[2]);
        self.application_version = DefaultingValue::Unique(version);
        self
    }
}
