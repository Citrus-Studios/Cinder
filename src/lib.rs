pub mod functions;
pub mod helper;
pub mod match_error_codes;
pub mod structure_type;

pub mod vk_instancing;

pub use mira::mem::zeroed_vec;

#[cfg(all(feature = "medium-logging", feature = "heavy-logging"))]
compile_error!(
    "`medium-logging` and `heavy-logging` are mutually exclusive and cannot be enabled together"
);

#[cfg(all(feature = "no-logging", feature = "heavy-logging"))]
compile_error!(
    "`no-logging` and `heavy-logging` are mutually exclusive and cannot be enabled together"
);

#[cfg(all(feature = "no-logging", feature = "medium-logging"))]
compile_error!(
    "`no-logging` and `heavy-logging` are mutually exclusive and cannot be enabled together"
);
