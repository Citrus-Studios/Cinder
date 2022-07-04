//! Builder for Application Info that creates a `SafeApplicationInfo`
//!
//! ```rust
//! use cinder::vk_instance::ApplicationInfoBuilder;
//!
//! let application_info = ApplicationInfoBuilder::new().
//! ```

use tracing::debug;

use crate::{
    functions::make_api_version, helper::DefaultingValue, vk_instancing::SafeApplicationInfo,
};

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
    /// With the defualt values of<br/>
    /// ```
    /// application_name: "Cinder Application"
    /// engine_name: "Cinder Engine"
    /// application_version: "1.0.0"
    /// engine_version: "1.0.0"
    /// api_version: "0.1.0.0"
    /// ```
    pub fn new() -> Self {
        debug!("Creating new `ApplicationInfoBuilder`");
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
        debug!("Added Application Name {}", name);
        self.application_name = DefaultingValue::Unique(String::from(name));
        self
    }
    /// Changes the Engine Name
    pub fn with_engine_name(mut self, engine_name: &str) -> Self {
        debug!("Added Engine Name {}", engine_name);
        self.engine_name = DefaultingValue::Unique(String::from(engine_name));
        self
    }
    /// Changes the Application Version using a u32
    pub fn with_application_version_u32(mut self, version: u32) -> Self {
        debug!("Added Application Version `u32` {}", version);
        self.application_version = DefaultingValue::Unique(version);
        self
    }
    /// Changes the Application Version using a str
    /// with the format of `X.X.X`<br/>
    /// Will panic if the format is supplied incorrectly
    pub fn with_application_version_str(mut self, version: &str) -> Self {
        debug!("Added Application Version `str` {}", version);
        // Split the application_version and make sure it contains 3 elements
        let version = version
            .split(".")
            .map(|x| {
                x.parse::<u8>()
                    .expect("You can not use negative version numbers")
            })
            .collect::<Vec<u8>>();
        assert!(3 == version.len());
        let version = make_api_version(0, version[0], version[1], version[2]);
        self.application_version = DefaultingValue::Unique(version);
        self
    }
    /// Changes the Engine Version using a u32
    pub fn with_engine_version_u32(mut self, version: u32) -> Self {
        debug!("Added Engine Version `u32` {}", version);
        self.engine_version = DefaultingValue::Unique(version);
        self
    }
    /// Changes the Engine Version using a str
    /// with the format of `X.X.X`<br/>
    /// Will panic if the format is supplied incorrectly
    pub fn with_engine_version_str(mut self, version: &str) -> Self {
        debug!("Added Engine Version `str` {}", version);
        // Split the application_version and make sure it contains 3 elements
        let version = version
            .split(".")
            .map(|x| {
                x.parse::<u8>()
                    .expect("You can not use negative version numbers")
            })
            .collect::<Vec<u8>>();
        assert!(3 == version.len());
        let version = make_api_version(0, version[0], version[1], version[2]);
        self.engine_version = DefaultingValue::Unique(version);
        self
    }
    /// Changes the Vulkan Version using a u32
    pub fn with_vulkan_version_u32(mut self, version: u32) -> Self {
        debug!("Added Vulkan Version `u32` {}", version);
        self.api_version = DefaultingValue::Unique(version);
        self
    }
    /// Changes the Vulkan Version using a str
    /// with the format of `X.X.X.X`<br/>
    /// Will panic if the format is supplied incorrectly
    pub fn with_vulkan_version_str(mut self, version: &str) -> Self {
        debug!("Added Vulkan Version `str` {}", version);
        // Split the application_version and make sure it contains 4 elements
        let version = version
            .split(".")
            .map(|x| {
                x.parse::<u8>()
                    .expect("You can not use negative version numbers")
            })
            .collect::<Vec<u8>>();
        assert!(4 == version.len());
        let version = make_api_version(version[0], version[1], version[2], version[3]);
        self.api_version = DefaultingValue::Unique(version);
        self
    }
    /// Builds the the Builder into the `SafeApplicationInfo`
    pub fn build(self) -> SafeApplicationInfo {
        debug!("Building `SafeApplicationInfo`");
        SafeApplicationInfo::new(
            self.application_name.unwrap().as_str(),
            self.engine_name.unwrap().as_str(),
            self.application_version.unwrap(),
            self.engine_version.unwrap(),
            self.api_version.unwrap(),
        )
    }
}
