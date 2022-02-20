use mira::vulkan::VkDevice;

pub struct LogicalDevice {
    device: VkDevice,
}

impl LogicalDevice {
    pub fn into_raw(self) -> VkDevice {
        todo!();
    }
}