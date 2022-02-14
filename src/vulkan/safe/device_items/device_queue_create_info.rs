use std::ptr;

use mira::vulkan::{VkDeviceQueueCreateInfo, VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO};

pub struct DeviceQueueCreateInfo<T> {
    next: Option<T>,
    flags: Option<u32>,
    queue_family_index: u32,
    queue_count: u32,
    queue_priorities: Option<Vec<f32>>,
}

pub struct DeviceQueueCreateInfoBuilder<T> {
    next: Option<T>,
    flags: Option<u32>,
    queue_family_index: Option<u32>,
    queue_count: Option<u32>,
    queue_priorities: Option<Vec<f32>>,
}

impl<T> DeviceQueueCreateInfo<T> {
    pub(crate) fn into_raw(self) -> VkDeviceQueueCreateInfo {
        return VkDeviceQueueCreateInfo {
            sType: VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
            pNext: match self.next {
                Some(next) => Box::into_raw(Box::new(next)) as *const _,
                None => ptr::null(),
            },
            flags: match self.flags {
                Some(flags) => flags,
                None => 0,
            },
            queueFamilyIndex: self.queue_family_index,
            queueCount: self.queue_count,
            pQueuePriorities: match self.queue_priorities {
                Some(queue_priorities) => queue_priorities.as_ptr(),
                None => ptr::null(),
            },
        };
    }
}

impl<T> DeviceQueueCreateInfoBuilder<T> {
    pub fn new() -> Self {
        DeviceQueueCreateInfoBuilder {
            next: None,
            flags: None,
            queue_family_index: None,
            queue_count: None,
            queue_priorities: None,
        }
    }

    pub fn next(mut self, next: T) -> Self {
        self.next = Some(next);
        
    }

    pub fn flags(mut self, flags: u32) -> Self {
        self.flags = Some(flags);
        
    }

    pub fn queue_family_index(mut self, queue_family_index: u32) -> Self {
        self.queue_family_index = Some(queue_family_index);
        
    }

    pub fn queue_count(mut self, queue_count: u32) -> Self {
        self.queue_count = Some(queue_count);
        self
    }

    pub fn queue_priorities(mut self, queue_priorities: Vec<f32>) -> Self {
        self.queue_priorities = Some(queue_priorities);
        self
    }

    pub fn build(self) -> DeviceQueueCreateInfo<T> {
        DeviceQueueCreateInfo {
            next: self.next,
            flags: self.flags,
            queue_family_index: self.queue_family_index.expect("Must provide a queue family index"),
            queue_count: self.queue_count.expect("Must provide a queue count for the queue family index."),
            queue_priorities: self.queue_priorities,
        }
    }
}