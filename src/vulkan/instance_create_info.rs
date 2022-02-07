use std::{ptr, ffi::CString};

use mira::vulkan::{VkInstanceCreateInfo, VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO, VK_MAKE_API_VERSION};

use super::application_info::{ApplicationInfo, ApplicationInfoBuilder};

#[test]
fn application_info_test() {
    let app_info = ApplicationInfoBuilder::<()>::new()
        .application_name("Triangle")
        .engine_name("None")
        .application_version(VK_MAKE_API_VERSION(0, 1, 0, 0))
        .engine_version(VK_MAKE_API_VERSION(0, 1, 0, 0))
        .build();
    let instance_create_info = InstanceCreateInfoBuilder::<(), ()>::new()
        .application_info(app_info)
        .build();
}

pub struct InstanceCreateInfo<'a, T, U> {
    pub pNext: Option<T>,
    pub flags: u32,
    pub pApplicationInfo: Option<ApplicationInfo<'a, U>>,
    pub ppEnabledLayerNames: Option<Vec<&'a str>>,
    pub ppEnabledExtensionNames: Option<Vec<&'a str>>,
}

pub struct InstanceCreateInfoBuilder<'a, T, U> {
    pub pNext: Option<T>,
    pub flags: Option<u32>,
    pub pApplicationInfo: Option<ApplicationInfo<'a, U>>,
    pub ppEnabledLayerNames: Option<Vec<&'a str>>,
    pub ppEnabledExtensionNames: Option<Vec<&'a str>>,
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
            enabledLayerCount: match self.ppEnabledLayerNames {
                Some(ref ppEnabledLayerNames) => ppEnabledLayerNames.len() as u32,
                None => 0,
            },
            ppEnabledLayerNames: match self.ppEnabledLayerNames {
                Some(ref ppEnabledLayerNames) => {
                    let mut new_vec = vec![];
                    for layer_name in ppEnabledLayerNames {
                        new_vec.push(CString::new(layer_name.as_bytes()).unwrap().as_ptr());
                    }
                    new_vec.as_ptr()
                },
                None => ptr::null(),
            },
            enabledExtensionCount: match self.ppEnabledExtensionNames {
                Some(ref ppEnabledExtensionNames) => ppEnabledExtensionNames.len() as u32,
                None => 0,
            },
            ppEnabledExtensionNames: match self.ppEnabledExtensionNames {
                Some(ref ppEnabledExtensionNames) => {
                    let mut new_vec = vec![];
                    for extension_name in ppEnabledExtensionNames {
                        new_vec.push(CString::new(extension_name.as_bytes()).unwrap().as_ptr());
                    }
                    new_vec.as_ptr()
                },
                None => ptr::null(),
            },
        };
    }
}

impl<'a, T, U> InstanceCreateInfoBuilder<'a, T, U> {
    pub fn new() -> Self {
        Self {
            pNext: None,
            flags: None,
            pApplicationInfo: None,
            ppEnabledLayerNames: None,
            ppEnabledExtensionNames: None,
        }
    }
    pub fn pNext(mut self, pNext: T) -> Self {
        self.pNext = Some(pNext);
        self
    }
    pub fn flags(mut self, flags: u32) -> Self {
        self.flags = Some(flags);
        self
    }
    pub fn application_info(mut self, application_info: ApplicationInfo<'a, U>) -> Self {
        self.pApplicationInfo = Some(application_info);
        self
    }
    pub fn enabled_layer_names(mut self, enabled_layer_names: Vec<&'a str>) -> Self {
        self.ppEnabledLayerNames = Some(enabled_layer_names);
        self
    }
    pub fn enabled_extensions(mut self, enabled_extensions: Vec<&'a str>) -> Self {
        self.ppEnabledExtensionNames = Some(enabled_extensions);
        self
    }
    pub fn build(self) -> InstanceCreateInfo<'a, T, U> {
        InstanceCreateInfo {
            pNext: self.pNext,
            flags: self.flags.unwrap_or(0),
            pApplicationInfo: self.pApplicationInfo,
            ppEnabledLayerNames: self.ppEnabledLayerNames,
            ppEnabledExtensionNames: self.ppEnabledExtensionNames,
        }
    }
}