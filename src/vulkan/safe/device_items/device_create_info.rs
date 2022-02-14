use super::{physical_device_features::PhysicalDeviceFeaturesBuilder, device_queue_create_info::DeviceQueueCreateInfo};

pub struct DeviceCreateInfo<'a, T> {
    next: Option<T>,
    flags: Option<u32>,
    queue_create_info_count: Option<u32>,
    queue_create_infos: Option<DeviceQueueCreateInfo<T>>,
    enabled_layer_count: Option<u32>,
    enabled_layer_names: Option<Vec<&'a str>>,
    enabled_extension_count: Option<u32>,
    enabled_extension_names: Option<Vec<&'a str>>,
    enabled_features: PhysicalDeviceFeaturesBuilder,
}

