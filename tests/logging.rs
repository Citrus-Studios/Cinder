use cinder::vk_instancing::vk_application_info::builder::ApplicationInfoBuilder;
use tracing::info;

#[tokio::test]
async fn basic_logging_test() {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    let _ = ApplicationInfoBuilder::new().build();
}
