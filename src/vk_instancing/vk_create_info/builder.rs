use tracing::debug;

use crate::{
    helper::DefaultingValue,
    vk_instancing::{
        vk_application_info::{builder::ApplicationInfoBuilder, r#final::ApplicationInfo},
        SafeCreateInfo,
    },
};

pub struct CreateInfoBuilder<'a> {
    pub application_info: DefaultingValue<&'a ApplicationInfo>,
    pub layer_names: DefaultingValue<Vec<String>>,
    pub enabled_extension_names: DefaultingValue<Vec<String>>,
}

impl<'a> CreateInfoBuilder<'a> {
    /// Creates a new `CreateInfoBuilder`
    /// With no layers or enabled extensions
    pub fn new() -> Self {
        #[cfg(any(feature = "medium-logging", feature = "heavy-logging"))]
        debug!("Created `CreateInfoBuilder`");
        let x = ApplicationInfoBuilder::new().build().finish();
        let x = &*Box::leak(Box::new(x));
        Self {
            application_info: DefaultingValue::Default(x),
            layer_names: DefaultingValue::Default(vec![]),
            enabled_extension_names: DefaultingValue::Default(vec![]),
        }
    }
    /// Changes the application info
    pub fn with_application_info(mut self, application_info: &'a ApplicationInfo) -> Self {
        #[cfg(feature = "detailed-logging")]
        debug!("Adding Application Name `{:#?}`", application_info);
        #[cfg(feature = "logging")]
        debug!("Adding Application Name");
        let x = self.application_info.unwrap();
        self.application_info = DefaultingValue::Unique(application_info);
        drop(x);
        self
    }
    /// Changes the layer names
    pub fn with_layer_names(mut self, layer_names: Vec<String>) -> Self {
        #[cfg(feature = "detailed-logging")]
        debug!("Adding Layer Names `{:#?}`", layer_names);
        #[cfg(feature = "logging")]
        debug!("Adding Layer Names");
        self.layer_names = DefaultingValue::Unique(layer_names);
        self
    }
    /// Changes the extensions
    pub fn with_extensions(mut self, extension_names: Vec<String>) -> Self {
        #[cfg(feature = "detailed-logging")]
        debug!("Adding Extensions `{:#?}`", extension_names);
        #[cfg(feature = "logging")]
        debug!("Adding Extensions");
        self.enabled_extension_names = DefaultingValue::Unique(extension_names);
        self
    }
    /// Builds the the Builder into the `SafeCreateInfo`
    pub fn build(self) -> SafeCreateInfo<'a> {
        #[cfg(any(feature = "detailed-logging", feature = "logging"))]
        debug!("Building `SafeCreateInfo`");
        let layer_names = self.layer_names.unwrap();
        let extensions = self.enabled_extension_names.unwrap();
        SafeCreateInfo::new_from(
            self.application_info.unwrap(),
            layer_names.len() as u32,
            layer_names,
            extensions.len() as u32,
            extensions,
        )
    }
}
