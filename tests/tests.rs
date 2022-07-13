use cinder::vk_instancing::{
    vk_application_info::{builder::ApplicationInfoBuilder, r#unsafe::UnsafeApplicationInfo},
    vk_create_info::builder::CreateInfoBuilder,
    UnsafeCreateInfo,
};
use mira::vulkan::{VkApplicationInfo, VkInstanceCreateInfo};

fn setup_logging() {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}

#[tokio::test]
async fn logging_test() {
    setup_logging();
    let application_info = ApplicationInfoBuilder::new().build().finish();
    let _create_info = CreateInfoBuilder::new()
        .with_application_info(&application_info)
        .build();
}

#[tokio::test]
async fn raw_conversion_test() {
    setup_logging();
    let application_info = ApplicationInfoBuilder::new().build();
    let unsafe_application_info: UnsafeApplicationInfo = application_info.clone().into();
    let _raw_application_info: VkApplicationInfo = unsafe_application_info.into();
    let application_info_finished = application_info.finish();

    let create_info = CreateInfoBuilder::new()
        .with_application_info(&application_info_finished)
        .build();
    let unsafe_create_info: UnsafeCreateInfo = create_info.clone().into();
    let _raw_create_info: VkInstanceCreateInfo = unsafe_create_info.into();
}

#[tokio::test]
async fn fundamental_struct_test() {
    setup_logging();
    let application_info = ApplicationInfoBuilder::new().build().finish();
    let _create_info = CreateInfoBuilder::new()
        .with_application_info(&application_info)
        .build()
        .finish();
}
