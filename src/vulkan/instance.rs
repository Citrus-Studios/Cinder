use std::{ptr, ffi::{CString}, sync::Arc};

use mira::{vulkan::{self as vk, VK_SUCCESS, VK_STRUCTURE_TYPE_APPLICATION_INFO}};
#[allow(unused_imports)]
use const_cstr::*;
use vk::{VkInstance, VK_MAKE_API_VERSION, VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO, VkApplicationInfo, VkInstanceCreateInfo};

use crate::vulkan::functions::vkCreateInstance;

use super::application_info::ApplicationInfoBuilder;

pub struct Instance {
    pub(crate) instance: VkInstance,
}

pub struct InstanceBuilder<'a> {
    application_name: Option<&'a str>, 
    engine_name: Option<&'a str>, 
    vulkan_version: Option<u32>, 
    application_version: Option<u32>,
    engine_version: Option<u32>,
}

impl Instance {
    pub fn new(
        application_name: Option<&str>, 
        engine_name: Option<&str>, 
        vulkan_version: Option<u32>, 
        application_version: Option<u32>,
        engine_version: Option<u32>,
    ) -> Arc<Self> {
        // Make the application info
        let application_name = application_name.unwrap_or("Cinder");
        let engine_name = engine_name.unwrap_or("None");
        let application_info = ApplicationInfoBuilder::<()>::new()
            .application_name(application_name)
            .engine_name(engine_name)
            .vulkan_version(vulkan_version.unwrap_or(VK_MAKE_API_VERSION(0, 1, 1, 0)))
            .application_version(application_version.unwrap_or(VK_MAKE_API_VERSION(0, 1, 0, 0)))
            .engine_version(engine_version.unwrap_or(VK_MAKE_API_VERSION(0, 1, 0, 0)))
            .build();
        
        // Make the instance create info
        let instance_create_info = VkInstanceCreateInfo {
            sType: VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
            pNext: ptr::null(),
            flags: 0,
            pApplicationInfo: &application_info as *const VkApplicationInfo,
            enabledLayerCount: 0,
            ppEnabledLayerNames: ptr::null(),
            enabledExtensionCount: 0,
            ppEnabledExtensionNames: ptr::null(),
        };

        let instance = ptr::null_mut();
        let instance_info = &instance_create_info as *const VkInstanceCreateInfo;
        let result = vkCreateInstance(instance_info, ptr::null(), instance);
        match result {
            VK_SUCCESS => {
                unsafe { Arc::new(Instance {
                    instance: *instance
                })}
            },
            _ => panic!("Failed to create instance"),
        }

        
    }
}

impl<'a> InstanceBuilder<'a> {
    pub fn new() -> Self {
        InstanceBuilder {
            application_name: None,
            engine_name: None,
            vulkan_version: None,
            application_version: None,
            engine_version: None,
        }
    }
    pub fn application_name(mut self, application_name: &'a str) -> Self {
        self.application_name = Some(application_name);
        self
    }
    pub fn engine_name(mut self, engine_name: &'a str) -> Self {
        self.engine_name = Some(engine_name);
        self
    }
    pub fn vulkan_version(mut self, variant: u8, major: u8, minor: u8, patch: u8) -> Self {
        self.vulkan_version = Some(VK_MAKE_API_VERSION(variant, major, minor, patch));
        self
    }
    pub fn application_version(mut self, variant: u8, major: u8, minor: u8, patch: u8) -> Self {
        self.application_version = Some(VK_MAKE_API_VERSION(variant, major, minor, patch));
        self
    }
    pub fn engine_version(mut self, variant: u8, major: u8, minor: u8, patch: u8) -> Self {
        self.engine_version = Some(VK_MAKE_API_VERSION(variant, major, minor, patch));
        self
    }
    pub fn build(self) -> Arc<Instance> {
        Instance::new(
            self.application_name,
            self.engine_name,
            self.vulkan_version,
            self.application_version,
            self.engine_version,
        )
    }
}
