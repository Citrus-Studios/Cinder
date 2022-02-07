pub mod triangle;
pub mod result;

pub mod vulkan {
    pub mod safe_wrapper {
        pub mod application_info;
        pub mod instance;
        pub mod physical_device;
        pub mod instance_create_info;
    }
    pub mod r#unsafe {
        pub mod unsafe_functions;
    }
}