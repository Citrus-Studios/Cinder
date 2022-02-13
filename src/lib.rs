pub mod triangle;
pub mod match_error_codes;

pub mod vulkan {
    pub mod safe {
        pub mod device_items {
            pub mod physical_device;
            pub mod logical_device;
            pub mod queue_family;
            pub mod device_queue_create_info;
            pub mod device_create_info;
            pub mod physical_device_features;
        }
        pub mod instance_items {
            pub mod application_info;
            pub mod instance;
            pub mod instance_create_info;
        }
        pub mod functions;
    }
    pub mod r#unsafe {
        pub mod unsafe_functions;
    }
}