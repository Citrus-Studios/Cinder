use mira::vulkan::{VkSurfaceKHR, VkWin32SurfaceCreateInfoKHR, VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR, VkDisplaySurfaceCreateInfoKHR};

use crate::vulkan::{r#unsafe::unsafe_functions::vkCreateWin32SurfaceKHR, safe::instance_items::instance::{Instance, self}};

pub struct SurfaceCreateInfo<T, U, V> {
    next: Option<T>,
    flags: Option<u32>,
    instance: U,
    window: Option<V>,
}

trait SpecificOS {
    type SurfaceCreateType;

    fn create_info(&self, instance: Instance) -> Self::SurfaceCreateType;
}

#[cfg(windows)]
impl<T, U, V> SpecificOS for SurfaceCreateInfo<T, U, V> {
    type SurfaceCreateType = VkWin32SurfaceCreateInfoKHR;

    fn create_info(&self, instance: Instance) -> VkWin32SurfaceCreateInfoKHR {
        return VkWin32SurfaceCreateInfoKHR {
            sType: VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR,
            pNext: match &self.next {
                Some(next) => Box::into_raw(Box::new(next)) as *const _,
                None => ptr::null(),
            },
            flags: match self.flags {
                Some(flags) => flags,
                None => 0,
            },
            hinstance: instance.hinstance,
            hwnd: match &self.window {
                Some(window) => window,
                None => ptr::null_mut(),
            },
        };
    }
}

#[cfg(feature = "wayland")]
impl<T, U, V> SpecificOS for SurfaceCreateInfo<T, U, V> {
    type SurfaceCreateType = VkWaylandSurfaceCreateInfoKHR;

    fn create_info(&self, instance: Instance) -> Self::SurfaceCreateType {
        return VkWin32SurfaceCreateInfoKHR {
            sType: VK_STRUCTURE_TYPE_WAYLAND_SURFACE_CREATE_INFO_KHR,
            pNext: match &self.next {
                Some(next) => Box::into_raw(Box::new(next)) as *const _,
                None => ptr::null(),
            },
            flags: match self.flags {
                Some(flags) => flags,
                None => 0,
            },
            hinstance: instance.hinstance,
            hwnd: match &self.window {
                Some(window) => window,
                None => ptr::null_mut(),
            },
        };
    }
}

#[cfg(feature = "xlib")]
impl<T, U, V> SpecificOS for SurfaceCreateInfo<T, U, V> {
    type SurfaceCreateType = VkXlibSurfaceCreateInfoKHR;

    fn create_info(&self, instance: Instance) -> Self::SurfaceCreateType {
        return VkWin32SurfaceCreateInfoKHR {
            sType: VK_STRUCTURE_TYPE_XLIB_SURFACE_CREATE_INFO_KHR,
            pNext: match &self.next {
                Some(next) => Box::into_raw(Box::new(next)) as *const _,
                None => ptr::null(),
            },
            flags: match self.flags {
                Some(flags) => flags,
                None => 0,
            },
            hinstance: instance.hinstance,
            hwnd: match &self.window {
                Some(window) => window,
                None => ptr::null_mut(),
            },
        };
    }
}

impl<T, U, V> SurfaceCreateInfo<T, U, V> {
    pub fn new(window_handle: U) -> Self {
        Self {
            next: None,
            flags: None,
            instance: window_handle,
            window: None,
        }
    }
    pub fn next(mut self, next: T) -> Self {
        self.next = Some(next);
        self
    }
    pub fn flags(mut self, flags: u32) -> Self {
        self.flags = Some(flags);
        self
    }
    pub fn instance(mut self, instance: U) -> Self {
        self.instance = instance;
        self
    }
    pub fn window(mut self, window: V) -> Self {
        self.window = Some(window);
        self
    }
    // pub fn build(self, instance: Instance) -> VkSurfaceKHR {
    //     let mut surface = 0;
    //     let create_info = self.create_info(instance);
    //     unsafe {
    //         vkCreateWin32SurfaceKHR(
    //             self.instance,
    //             self,
    //             ptr::null(),
    //             &mut surface,
    //             instance.instance,
    //         );
    //     }
    //     &surface
    // }
}