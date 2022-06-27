/// # Taken from https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_MAKE_VERSION.html<br/>
/// Creates a Vulkan version from 3 numbers
#[deprecated(note = "Deprecated in vulkan please use `make_api_version` instead")]
pub fn make_version(major: u8, minor: u8, patch: u8) -> u32 {
    (major as u32) << 22 | (minor as u32) << 12 | patch as u32
}
/// # Taken from https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_MAKE_API_VERSION.html<br/>
/// Creates a Vulkan version from 4 numbers
pub fn make_api_version(variant: u8, major: u8, minor: u8, patch: u8) -> u32 {
    (variant as u32) << 29 | (major as u32) << 22 | (minor as u32) << 12 | (patch as u32)
}
