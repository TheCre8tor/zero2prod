use tracing::{subscriber::set_global_default, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

pub struct Telemetry;

impl Telemetry {
    /// Compose multiple layers into a `tracing`'s subscriber.
    ///
    /// # Implementation Notes
    ///
    /// We are using `impl Subscriber` as return type to avoid having to
    /// spell out the actual type of the returned subscriber, which is
    /// indeed quite complex.
    /// We need to explicitly call out that the returned subscriber is
    /// `Send` and `Sync` to make it possible to pass it to `init_subscriber`
    /// later on.
    fn get_subscriber(name: String, env_filter: String) -> impl Subscriber + Send + Sync {
        let env_filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new(env_filter));
        let formatting_layer = BunyanFormattingLayer::new(name, std::io::stdout);

        Registry::default()
            .with(env_filter)
            .with(JsonStorageLayer)
            .with(formatting_layer)
    }

    /// Register a subscriber as global default to process span data.
    ///
    /// It should only be called once!
    pub fn init_subscriber(application_name: String) -> () {
        LogTracer::init().expect("Failed to set logger.");
        let subscriber = Telemetry::get_subscriber(application_name, "info".into());
        set_global_default(subscriber).expect("Failed to set subscriber");
    }
}
