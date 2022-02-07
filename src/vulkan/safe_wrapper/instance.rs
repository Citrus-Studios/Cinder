use std::{sync::Arc, mem::zeroed, ptr};

use mira::{vulkan::{self as vk, VK_SUCCESS}};
#[allow(unused_imports)]
use const_cstr::*;
use vk::{VkInstance, VK_MAKE_API_VERSION, VkInstanceCreateInfo};
use crate::result::{CinderResult::{OkC, ErrC}, MatchErrorCode};


use super::{application_info::ApplicationInfoBuilder, instance_create_info::InstanceCreateInfoBuilder, functions::create_instance};

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
        let instance_create_info = InstanceCreateInfoBuilder::<(), ()>::new()
            .application_info(application_info)
            .build();

        let instance_result = create_instance(Some(instance_create_info.into_raw()), None);
        match instance_result {
            OkC(instance) => Arc::new(Instance { instance }),
            ErrC(error) => panic!("Failed to create instance: {:?}", error.match_error_code()),
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
