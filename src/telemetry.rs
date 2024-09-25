use tracing::{subscriber::set_global_default, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{fmt::MakeWriter, layer::SubscriberExt, EnvFilter, Registry};

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
    fn get_subscriber<Sink>(
        name: String,
        env_filter: String,
        sink: Sink,
    ) -> impl Subscriber + Sync + Send
    where
        // higher-ranked trait bound (HRTB)
        Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
    {
        let env_filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new(env_filter));
        let formatting_layer = BunyanFormattingLayer::new(name, sink);

        Registry::default()
            .with(env_filter)
            .with(JsonStorageLayer)
            .with(formatting_layer)
    }

    /// Register a subscriber as global default to process span data.
    ///
    /// It should only be called once!
    pub fn init_subscriber<Sink>(application_name: String, env_filter: String, sink: Sink)
    where
        Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
    {
        LogTracer::init().expect("Failed to set logger.");
        let subscriber = Telemetry::get_subscriber(application_name, env_filter, sink);
        set_global_default(subscriber).expect("Failed to set subscriber");
    }
}
