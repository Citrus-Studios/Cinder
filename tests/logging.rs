use cinder::vk_instancing::vk_application_info::builder::ApplicationInfoBuilder;
use tracing_test::traced_test;

#[tokio::test]
#[traced_test]
async fn logging_test() {
    let _ = ApplicationInfoBuilder::new().build();
}
