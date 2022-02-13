

pub struct LogicalDevice {

}

impl LogicalDevice {
    pub fn create_logical_device(physical_device: PhysicalDevice) {
        let indices: QueueFamilyIndices = findQueueFamilies(physical_device.current_physical_device);
    }
}