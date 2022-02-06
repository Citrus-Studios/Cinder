use std::ptr;

use mira::vulkan::{VkInstanceCreateInfo, VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO};

use super::application_info::ApplicationInfo;

pub struct InstanceCreateInfo<'a, T, U> {
    pub pNext: Option<T>,
    pub flags: u32,
    pub pApplicationInfo: Option<&'a ApplicationInfo<'a, U>>,
    pub enabledLayerCount: u32,
    pub ppEnabledLayerNames: Option<&'a str>,
    pub enabledExtensionCount: u32,
    pub ppEnabledExtensionNames: Option<&'a str>,
}

pub struct InstanceCreateInfoBuilder<'a, T, U> {
    pub pNext: Option<T>,
    pub flags: Option<u32>,
    pub pApplicationInfo: Option<&'a ApplicationInfo<'a, U>>,
    pub enabledLayerCount: Option<u32>,
    pub ppEnabledLayerNames: Option<&'a str>,
    pub enabledExtensionCount: Option<u32>,
    pub ppEnabledExtensionNames: Option<&'a str>,
}

impl<'a, T, U> InstanceCreateInfo<'a, T, U> {
    pub fn into_raw(self) -> VkInstanceCreateInfo {
        return VkInstanceCreateInfo {
            sType: VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
            pNext: match self.pNext {
                Some(pNext) => Box::into_raw(Box::new(pNext)) as *const _,
                None => ptr::null(),
            },
            flags: self.flags,
            pApplicationInfo: match self.pApplicationInfo {
                Some(pApplicationInfo) => Box::into_raw(Box::new(pApplicationInfo.into_raw())) as *const _,
                None => ptr::null(),
            },
            enabledLayerCount: self.enabledLayerCount,
            ppEnabledLayerNames: match self.ppEnabledLayerNames {
                Some(ppEnabledLayerNames) => ppEnabledLayerNames.as_ptr(),
                None => ptr::null(),
            },
            enabledExtensionCount: 0,
            ppEnabledExtensionNames: ptr::null(),
        };
    }
}