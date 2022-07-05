/// Unsafe version of `SafeApplicationInfo`
pub struct UnsafeApplicationInfo {
    pub application_name: *const char,
    pub engine_name: *const char,
    pub application_version: u32,
    pub engine_version: u32,
    pub api_version: u32,
}

impl UnsafeApplicationInfo {
    pub unsafe fn new(
        application_name: *const char,
        engine_name: *const char,
        application_version: u32,
        engine_version: u32,
        api_version: u32,
    ) -> Self {
        Self {
            application_name,
            engine_name,
            application_version,
            engine_version,
            api_version,
        }
    }
}
