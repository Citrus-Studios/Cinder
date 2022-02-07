#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use lazy_static::lazy_static;

use mira::{vulkan::{PFN_vkCreateInstance, PFN_vkEnumeratePhysicalDevices, VkInstanceCreateInfo, VkAllocationCallbacks, VkInstance, VkResult, VkPhysicalDevice}, loader};
use const_cstr::*;
use paste::paste;

use crate::result::CinderResult;

macro_rules! vk_instance {(
$(
    $( #[$attr:meta] )*
    $pub:vis fn $fname:ident ( $($arg_name:ident : $ArgTy:ty),* $(,)? )$ (-> $RetTy:ty)?;
)*) => (paste! {
    $(
        $( #[$attr] )*
        $pub fn $fname ( $( $arg_name: $ArgTy ),* ) $(-> $RetTy)?
        {
            lazy_static! {
                static ref $fname: [< PFN_ $fname >] = unsafe {
                    loader::instance(
                        std::ptr::null_mut(),
                        const_cstr!(stringify!( $fname )),
                    ).expect(concat!(
                        "Failed to load `", stringify!($fname), "`",
                    ))
                };
            }
            unsafe { $fname( $($arg_name),* ) }
        }
    )*
})}


macro_rules! vk_device {(
$(
    $( #[$attr:meta] )*
    $pub:vis fn $fname:ident ( $($arg_name:ident : $ArgTy:ty),* $(,)? )$ (-> $RetTy:ty)?;
)*) => (paste! {
    $(
        $( #[$attr] )*
        $pub fn $fname ( $( $arg_name: $ArgTy ),* )$ (-> $RetTy)?
        {
            lazy_static! {
                static ref $fname: [< PFN_ $fname >] = unsafe {
                    loader::device(
                        std::ptr::null_mut(),
                        const_cstr!(stringify!( $fname )),
                    ).expect(concat!(
                        "Failed to load `", stringify!($fname), "`",
                    ))
                };
            }
            unsafe { $fname( $($arg_name),* ) }
        }
    )*
})}

vk_instance!(
    pub(crate) fn vkCreateInstance(pCreateInfo: *const VkInstanceCreateInfo, pAllocator: *const VkAllocationCallbacks, pInstance: *mut VkInstance) -> VkResult;
);

vk_device!(
    pub(crate) fn vkEnumeratePhysicalDevices(instance: VkInstance, pPhysicalDeviceCount: *mut u32, pPhysicalDevices: *mut VkPhysicalDevice) -> VkResult;
);