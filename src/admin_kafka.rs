use rdkafka::{admin::AdminClient, client::DefaultClientContext, ClientConfig};

pub fn create_config() -> ClientConfig {
    let mut config = ClientConfig::new();
    config
        .set("bootstrap.servers", "localhost:29092")
        .set("client.id", "rdkafka_integration_test_client")
        .set("session.timeout.ms", "6000")
        .set("auto.offset.reset", "earliest")
        .set("heartbeat.interval.ms", "3000");
    config
}

pub fn create_admin_client() -> AdminClient<DefaultClientContext> {
    create_config().create().expect("Admin creation failed")
}
