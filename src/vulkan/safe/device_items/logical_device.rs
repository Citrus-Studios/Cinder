use super::{queue_family::QueueFamily, physical_device::PhysicalDevice};

pub struct LogicalDevice {

}

impl LogicalDevice {
    pub fn create_logical_device(physical_device: PhysicalDevice) {
        let indices = QueueFamily::new().select_queue_family(physical_device);
    }
}