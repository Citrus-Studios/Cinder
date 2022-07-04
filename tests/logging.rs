use cinder::vk_instancing::vk_application_info::builder::ApplicationInfoBuilder;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use tracing_test::traced_test;

#[tokio::test]
#[traced_test]
async fn logging_test() {
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let _ = ApplicationInfoBuilder::new().build();
}
