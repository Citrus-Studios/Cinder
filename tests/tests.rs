use cinder::vk_instancing::{
    vk_application_info::builder::ApplicationInfoBuilder,
    vk_create_info::builder::CreateInfoBuilder,
};

#[tokio::test]
async fn basic_logging_test() {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    let application_info = ApplicationInfoBuilder::new().build();
    let _create_info = CreateInfoBuilder::new()
        .with_application_info(&application_info)
        .build();
}
