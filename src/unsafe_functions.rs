#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(unused_imports)]

use const_cstr::*;
use mira::{loader, vulkan::*};
use paste::paste;

// Creates a "safe" function for the unsafe function
macro_rules! vk_instance {(
$(
    $( #[$attr:meta] )*
    $pub:vis fn $fname:ident ( $($arg_name:ident : $ArgTy:ty),* $(,)? )$ (-> $RetTy:ty)?;
)*) => (paste! {
    $(
        $( #[$attr] )*
        $pub fn $fname( $( $arg_name: $ArgTy ),*, instance: Option<VkInstance> ) $(-> $RetTy)? {
            let $fname: [<PFN_$fname>] = unsafe {
                loader::instance(
                    instance.unwrap_or(std::ptr::null_mut()),
                    const_cstr!(stringify!( $fname )),
                ).expect(concat!(
                    "Failed to load `", stringify!($fname), "`",
                ))
            };
            unsafe { $fname( $($arg_name),* ) }
        }
    )*
})}

vk_instance!(
    pub(crate) fn vkCreateInstance(
        pCreateInfo: *const VkInstanceCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pInstance: *mut VkInstance,
    ) -> VkResult;
    pub(crate) fn vkEnumeratePhysicalDevices(
        instance: VkInstance,
        pPhysicalDeviceCount: *mut u32,
        pPhysicalDevices: *mut VkPhysicalDevice,
    ) -> VkResult;
    pub(crate) fn vkGetPhysicalDeviceProperties(
        physicalDevice: VkPhysicalDevice,
        pProperties: *mut VkPhysicalDeviceProperties,
    );
    pub(crate) fn vkGetPhysicalDeviceQueueFamilyProperties(
        physicalDevice: VkPhysicalDevice,
        pQueueFamilyPropertyCount: *mut u32,
        pQueueFamilyProperties: *mut VkQueueFamilyProperties,
    );
    pub(crate) fn vkGetPhysicalDeviceSurfaceSupportKHR(
        physicalDevice: VkPhysicalDevice,
        queueFamilyIndex: u32,
        surface: VkSurfaceKHR,
        pSupported: *mut VkBool32,
    ) -> VkResult;
    pub(crate) fn vkCreateDevice(
        physicalDevice: VkPhysicalDevice,
        pCreateInfo: *const VkDeviceCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pDevice: *mut VkDevice,
    ) -> VkResult;
    pub(crate) fn vkCreateWin32SurfaceKHR(
        instance: VkInstance,
        pCreateInfo: *const VkWin32SurfaceCreateInfoKHR,
        pAllocator: *const VkAllocationCallbacks,
        pSurface: *mut VkSurfaceKHR,
    ) -> VkResult;

    pub(crate) fn vkDestroyInstance(instance: VkInstance, pAllocator: *const VkAllocationCallbacks);
);
