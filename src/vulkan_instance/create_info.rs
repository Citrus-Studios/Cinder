use super::application_info::SafeApplicationInfo;

/// A Safe Wrapper for Create Info
pub struct SafeCreateInfo<'a> {
    pub application_info: &'a SafeApplicationInfo,
}

impl<'a> SafeCreateInfo<'a> {
    pub fn new(application_info: &'a SafeApplicationInfo) -> Self {
        Self { application_info }
    }
}
