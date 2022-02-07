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
    pNext: Option<T>,
    pApplicationName: Option<&'a str>,
    applicationVersion: u32,
    pEngineName: Option<&'a str>,
    engineVersion: u32,
    apiVersion: u32,
}

pub struct ApplicationInfoBuilder<'a, T> {
    pNext: Option<T>,
    pApplicationName: Option<&'a str>,
    applicationVersion: Option<u32>,
    pEngineName: Option<&'a str>,
    engineVersion: Option<u32>,
    apiVersion: Option<u32>,
}


impl<'a, T> ApplicationInfo<'a, T> {
    pub(crate) fn into_raw(self) -> VkApplicationInfo {
        let app_info: VkApplicationInfo = VkApplicationInfo {
            sType: VK_STRUCTURE_TYPE_APPLICATION_INFO,
            pNext: match self.pNext {
                Some(pNext) => Box::into_raw(Box::new(pNext)) as *const _,
                None => ptr::null(),
            },
            pApplicationName: match self.pApplicationName {
                Some(pApplicationName) => {
                    let x = CString::new(pApplicationName.as_bytes()).unwrap();
                    x.as_ptr()
                }
                None => {
                    let x = CString::new("Cinder".as_bytes()).unwrap();
                    x.as_ptr()
                },
            },
            applicationVersion: self.applicationVersion,
            pEngineName: match self.pEngineName {
                Some(pEngineName) => {
                    let x = CString::new(pEngineName.as_bytes()).unwrap();
                    x.as_ptr()
                }
                None => {
                    let x = CString::new("Cinder".as_bytes()).unwrap();
                    x.as_ptr()
                },
            },
            engineVersion: self.engineVersion,
            apiVersion: self.apiVersion,
        };
        return app_info;
    }
}

impl<'a, T> ApplicationInfoBuilder<'a, T> {
    pub fn new() -> Self {
        ApplicationInfoBuilder {
            pNext: None,
            pApplicationName: None,
            applicationVersion: None,
            pEngineName: None,
            engineVersion: None,
            apiVersion: None,
        }
    }
    pub fn pNext(mut self, pNext: T) -> Self {
        self.pNext = Some(pNext);
        self
    }
    pub fn application_name(mut self, pApplicationName: &'a str) -> Self {
        self.pApplicationName = Some(pApplicationName);
        self
    }
    pub fn application_version(mut self, applicationVersion: u32) -> Self {
        self.applicationVersion = Some(applicationVersion);
        self
    }
    pub fn engine_name(mut self, pEngineName: &'a str) -> Self {
        self.pEngineName = Some(pEngineName);
        self
    }
    pub fn engine_version(mut self, engineVersion: u32) -> Self {
        self.engineVersion = Some(engineVersion);
        self
    }
    pub fn vulkan_version(mut self, apiVersion: u32) -> Self {
        self.apiVersion = Some(apiVersion);
        self
    }
    pub fn build(self) -> ApplicationInfo<'a, T> {
        ApplicationInfo {
            pNext: self.pNext,
            pApplicationName: self.pApplicationName,
            applicationVersion: self.applicationVersion.unwrap_or(VK_MAKE_API_VERSION(0, 1, 0, 0)),
            pEngineName: self.pEngineName,
            engineVersion: self.engineVersion.unwrap_or(VK_MAKE_API_VERSION(0, 1, 0, 0)),
            apiVersion: self.apiVersion.unwrap_or(VK_MAKE_API_VERSION(0, 1, 0, 0)),
        }
    }
}