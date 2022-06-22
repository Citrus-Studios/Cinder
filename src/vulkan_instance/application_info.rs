/// Safe Wrapper for Application Info to prevent unsafe headaches.
pub struct SafeApplicationInfo {
    pub application_name: String,
    pub engine_name: String,
    pub application_version: u32,
    pub engine_version: u32,
    pub api_version: u32,
}

impl SafeApplicationInfo {
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
}
