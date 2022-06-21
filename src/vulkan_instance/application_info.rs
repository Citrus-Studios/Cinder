/// Safe Wrapper for Application Info to prevent unsafe headaches.
pub struct SafeApplicationInfo {
    application_name: String,
    engine_name: String,
    application_version: u32,
    engine_version: u32,
    api_version: u32,
}

impl SafeApplicationInfo {
    pub fn new() -> Self {
        Self {}
    }
}
