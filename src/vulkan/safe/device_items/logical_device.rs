use mira::vulkan::VkDevice;

pub struct LogicalDevice {
    device: VkDevice,
}

pub struct LogicalDeviceBuilder {
    
}

impl LogicalDevice {
    pub fn into_raw(self) -> VkDevice {
        todo!();
    }
}