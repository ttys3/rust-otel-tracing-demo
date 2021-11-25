use log;
use opentelemetry::{global, sdk::export::trace::stdout, trace::Tracer};
use tracing::{error, span};
// use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

fn main() {
    env_logger::init();
    // LogTracer::init().expect("Failed to set logger");
    // let env_filter = EnvFilter::try_from_default_env().unwrap();

    // Install a new OpenTelemetry trace pipeline
    let stdout_tracer = stdout::new_pipeline()
        .with_pretty_print(true)
        .install_simple();

    // Create a jaeger exporter pipeline for a `trace_demo` service.
    let jaeger_tracer = opentelemetry_jaeger::new_pipeline()
        .with_service_name("trace_demo")
        .with_collector_endpoint("http://localhost:14268/api/traces")
        .install_simple()
        .expect("Error initializing Jaeger exporter");

    // Create a tracing layer with the configured tracer
    let telemetry = tracing_opentelemetry::layer()
        .with_tracer(stdout_tracer)
        .with_tracer(jaeger_tracer);

    // Use the tracing subscriber `Registry`, or any other subscriber
    // that impls `LookupSpan`
    let subscriber = Registry::default().with(telemetry)/*.with(env_filter)*/;

    log::info!("iiii this is info level log");
    log::error!("eeee this is error level log");

    // Trace executed code
    tracing::subscriber::with_default(subscriber, || {
        // Spans will be sent to the configured OpenTelemetry exporter
        let root = span!(tracing::Level::TRACE, "app_start", work_units = 2);
        let _enter = root.enter();

        error!("xxxx go to root span.");
    });

    // Shutdown trace pipeline
    global::shutdown_tracer_provider();
}
