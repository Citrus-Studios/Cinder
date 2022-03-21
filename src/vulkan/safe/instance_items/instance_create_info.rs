#![allow(non_snake_case)]
use std::{ffi::CString, ptr};

use mira::vulkan::{VkInstanceCreateInfo, VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO};

use super::application_info::ApplicationInfo;

#[derive(Clone)]
pub struct InstanceCreateInfo<T, U> {
    pub pNext: Option<T>,
    pub flags: u32,
    pub pApplicationInfo: Option<ApplicationInfo<U>>,
    pub ppEnabledLayerNames: Option<Vec<CString>>,
    pub ppEnabledExtensionNames: Option<Vec<CString>>,
}

pub struct InstanceCreateInfoBuilder<T, U> {
    pub pNext: Option<T>,
    pub flags: Option<u32>,
    pub pApplicationInfo: Option<ApplicationInfo<U>>,
    pub ppEnabledLayerNames: Option<Vec<String>>,
    pub ppEnabledExtensionNames: Option<Vec<String>>,
}

impl<T, U> InstanceCreateInfo<T, U> {
    pub fn into_raw(self) -> VkInstanceCreateInfo {
        return VkInstanceCreateInfo {
            sType: VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
            pNext: match self.pNext {
                Some(pNext) => Box::into_raw(Box::new(pNext)) as *const _,
                None => ptr::null(),
            },
            flags: self.flags,
            pApplicationInfo: match self.pApplicationInfo {
                Some(pApplicationInfo) => {
                    Box::into_raw(Box::new(pApplicationInfo.into_raw())) as *const _
                }
                None => ptr::null(),
            },
            enabledLayerCount: match self.ppEnabledLayerNames {
                Some(ref ppEnabledLayerNames) => ppEnabledLayerNames.len() as u32,
                None => 0,
            },
            ppEnabledLayerNames: match self.ppEnabledLayerNames {
                Some(ref ppEnabledLayerNames) => {
                    ppEnabledLayerNames.into_iter().map(|x| {
                        x.as_ptr() as *const i8
                    }).collect::<Vec<*const i8>>().as_ptr()
                },
                None => ptr::null(),
            },
            enabledExtensionCount: match self.ppEnabledExtensionNames.clone() {
                Some(ref ppEnabledExtensionNames) => ppEnabledExtensionNames.len() as u32,
                None => 0,
            },
            ppEnabledExtensionNames: match self.ppEnabledExtensionNames.clone() {
                Some(ref ppEnabledExtensionNames) => {
                    ppEnabledExtensionNames.into_iter().map(|x| {
                        x.as_ptr() as *const i8
                    }).collect::<Vec<*const i8>>().as_ptr()
                },
                None => ptr::null(),
            },
        };
    }
}

impl<T, U> InstanceCreateInfoBuilder<T, U> {
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
    pub fn application_info(mut self, application_info: ApplicationInfo<U>) -> Self {
        self.pApplicationInfo = Some(application_info);
        self
    }
    pub fn enabled_layer_names(mut self, enabled_layer_names: Option<Vec<String>>) -> Self {
        self.ppEnabledLayerNames = enabled_layer_names;
        self
    }
    pub fn enabled_extensions(mut self, enabled_extensions: Option<Vec<String>>) -> Self {
        self.ppEnabledExtensionNames = enabled_extensions;
        self
    }
    pub fn build(self) -> InstanceCreateInfo<T, U> {
        InstanceCreateInfo {
            pNext: self.pNext,
            flags: self.flags.unwrap_or(0),
            pApplicationInfo: self.pApplicationInfo,
            ppEnabledLayerNames: match self.ppEnabledLayerNames {
                Some(ppEnabledLayerNames) => Some(
                    ppEnabledLayerNames
                        .into_iter()
                        .map(|x| CString::new(x.as_bytes()).unwrap())
                        .collect(),
                ),
                None => None,
            },
            ppEnabledExtensionNames: match self.ppEnabledExtensionNames {
                Some(ref ppEnabledExtensionNames) => Some(
                    ppEnabledExtensionNames
                        .into_iter()
                        .map(|x| CString::new(x.as_bytes()).unwrap())
                        .collect(),
                ),
                None => None,
            },
        }
    }
}
