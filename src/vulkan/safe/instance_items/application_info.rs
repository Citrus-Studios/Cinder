#![allow(non_snake_case)]
use std::{ptr, ffi::CString};

use mira::vulkan::{VkApplicationInfo, VK_STRUCTURE_TYPE_APPLICATION_INFO, VK_MAKE_API_VERSION};

#[test]
fn application_info_test() {
    let app_info = ApplicationInfoBuilder::<()>::new()
        .application_name("Triangle")
        .engine_name("None")
        .application_version(VK_MAKE_API_VERSION(0, 1, 0, 0))
        .engine_version(VK_MAKE_API_VERSION(0, 1, 0, 0))
        .build();
    let _raw_app_info = app_info.into_raw();
}


pub struct ApplicationInfo<'a, T> {
    next: Option<T>,
    application_name: Option<&'a str>,
    application_version: u32,
    engine_name: Option<&'a str>,
    engine_version: u32,
    vulkan_version: u32,
}

pub struct ApplicationInfoBuilder<'a, T> {
    next: Option<T>,
    application_name: Option<&'a str>,
    application_version: Option<u32>,
    engine_name: Option<&'a str>,
    engine_version: Option<u32>,
    vulkan_version: Option<u32>,
}


impl<'a, T> ApplicationInfo<'a, T> {
    pub(crate) fn into_raw(self) -> VkApplicationInfo {
        let app_info: VkApplicationInfo = VkApplicationInfo {
            sType: VK_STRUCTURE_TYPE_APPLICATION_INFO,
            pNext: match self.next {
                Some(next) => Box::into_raw(Box::new(next)) as *const _,
                None => ptr::null(),
            },
            pApplicationName: match self.application_name {
                Some(application_name) => {
                    let x = CString::new(application_name.as_bytes()).unwrap();
                    x.as_ptr()
                }
                None => {
                    let x = CString::new("Cinder".as_bytes()).unwrap();
                    x.as_ptr()
                },
            },
            applicationVersion: self.application_version,
            pEngineName: match self.engine_name {
                Some(engine_name) => {
                    let x = CString::new(engine_name.as_bytes()).unwrap();
                    x.as_ptr()
                }
                None => {
                    let x = CString::new("Cinder".as_bytes()).unwrap();
                    x.as_ptr()
                },
            },
            engineVersion: self.engine_version,
            apiVersion: self.vulkan_version,
        };
        return app_info;
    }
}

impl<'a, T> ApplicationInfoBuilder<'a, T> {
    pub fn new() -> Self {
        ApplicationInfoBuilder {
            next: None,
            application_name: None,
            application_version: None,
            engine_name: None,
            engine_version: None,
            vulkan_version: None,
        }
    }
    pub fn next(mut self, next: T) -> Self {
        self.next = Some(next);
        self
    }
    pub fn application_name(mut self, application_name: &'a str) -> Self {
        self.application_name = Some(application_name);
        self
    }
    pub fn application_version(mut self, application_version: u32) -> Self {
        self.application_version = Some(application_version);
        self
    }
    pub fn engine_name(mut self, engine_name: &'a str) -> Self {
        self.engine_name = Some(engine_name);
        self
    }
    pub fn engine_version(mut self, engine_version: u32) -> Self {
        self.engine_version = Some(engine_version);
        self
    }
    pub fn vulkan_version(mut self, vulkan_version: u32) -> Self {
        self.vulkan_version = Some(vulkan_version);
        self
    }
    pub fn build(self) -> ApplicationInfo<'a, T> {
        ApplicationInfo {
            next: self.next,
            application_name: self.application_name,
            application_version: self.application_version.unwrap_or(VK_MAKE_API_VERSION(0, 1, 0, 0)),
            engine_name: self.engine_name,
            engine_version: self.engine_version.unwrap_or(VK_MAKE_API_VERSION(0, 1, 0, 0)),
            vulkan_version: self.vulkan_version.unwrap_or(VK_MAKE_API_VERSION(0, 1, 0, 0)),
        }
    }
}