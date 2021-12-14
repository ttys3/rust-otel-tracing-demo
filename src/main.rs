mod util;

// use opentelemetry::sdk::export::trace::stdout;
use opentelemetry::global;
use tracing::{error, info, span, warn};
// use tracing_log::LogTracer;
use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter, Registry};

#[allow(unused_imports)]
use tracing_subscriber::prelude::*;

fn main() {
    // To convert log::Records as tracing::Events, set LogTracer as the default logger by calling its init or init_with_filter methods
    // https://docs.rs/tracing-log/latest/tracing_log/#usage
    // https://docs.rs/tracing-log/latest/src/tracing_log/lib.rs.html#58
    // //! ## Caution: Mixing both conversions
    //!
    //! Note that logger implementations that convert log records to trace events
    //! should not be used with `Subscriber`s that convert trace events _back_ into
    //! log records (such as the `TraceLogger`), as doing so will result in the
    //! event recursing between the subscriber and the logger forever (or, in real
    //! life, probably overflowing the call stack).
    //!
    //! If the logging of trace events generated from log records produced by the
    //! `log` crate is desired, either the `log` crate should not be used to
    //! implement this logging, or an additional layer of filtering will be
    //! required to avoid infinitely converting between `Event` and `log::Record`.
    // tracing_log::LogTracer::init().expect("Failed to set logger");

    // Install a new OpenTelemetry trace pipeline
    // let stdout_tracer = stdout::new_pipeline()
    //     .with_pretty_print(true)
    //     .install_simple();

    // no need init Logger here, SubscriberInitExt.init will init the Logger
    // env_logger::init();
    // util::init_env_logger();

    // this will set global default and init logger
    // tracing_subscriber::fmt::init();

    // Create a jaeger exporter pipeline for a `trace_demo` service.
    let jaeger_tracer = opentelemetry_jaeger::new_pipeline()
        .with_service_name("trace_demo")
        .with_collector_endpoint("http://localhost:14268/api/traces")
        .install_simple()
        .expect("Error initializing Jaeger exporter");

    // Create a tracing layer with the configured tracer
    let telemetry = tracing_opentelemetry::layer()
        // .with_tracer(stdout_tracer)
        .with_tracer(jaeger_tracer);

    // let format = fmt::format()
    //     .pretty()
    //     .with_source_location(true);

    // warning: when using `LocalTime`, you'll need to build with `RUSTFLAGS="--cfg unsound_local_offset"` otherwise not tracing log will shown
    // see https://github.com/tokio-rs/tracing/pull/1699
    let fmt_layer = fmt::layer().with_timer(fmt::time::LocalTime::rfc_3339());

    let filter_layer = EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new("info")).unwrap();

    // Use the tracing subscriber `Registry`, or any other subscriber
    // that impls `LookupSpan`
    // https://docs.rs/tracing-subscriber/0.3.3/tracing_subscriber/fmt/index.html#composing-layers
    // init() will set_global_default() and init tracing-log (also init default Logger)
    // see https://docs.rs/tracing-subscriber/0.3.3/tracing_subscriber/util/trait.SubscriberInitExt.html#method.init
    Registry::default().with(filter_layer).with(fmt_layer).with(telemetry).init(); /*.with(env_filter)*/

    // if let Err(err) = tracing::subscriber::set_global_default(subscriber) {
    //     panic!("setting tracing default subscriber failed, err={}", err)
    // }

    log::trace!("I: this is trace level log");
    log::info!("II: this is info level log");
    log::error!("III: this is error level log");

    // Trace executed code
    // tracing::subscriber::with_default(subscriber, || {
    //     // Spans will be sent to the configured OpenTelemetry exporter
    //     let root = span!(tracing::Level::TRACE, "app_start", work_units = 2);
    //     let _enter = root.enter();
    //
    //     error!("xxxx go to root span.");
    // });

    {
        // Spans will be sent to the configured OpenTelemetry exporter
        let root = span!(tracing::Level::TRACE, "app_start", work_units = 2);
        let _enter = root.enter();

        std::thread::sleep(std::time::Duration::from_millis(20));
        error!("xxxx start root span. cost=20ms");
        {
            // Spans will be sent to the configured OpenTelemetry exporter
            let api1 = span!(tracing::Level::TRACE, "api_call_1", work_units = 4);
            let _enter = api1.enter();

            // slow call
            std::thread::sleep(std::time::Duration::from_millis(40));
            warn!("xxxx call api1 done. cost=40ms");
            {
                // Spans will be sent to the configured OpenTelemetry exporter
                let api2 = span!(tracing::Level::TRACE, "api_call_2", work_units = 4);
                let _enter = api2.enter();

                // slow call
                std::thread::sleep(std::time::Duration::from_millis(80));
                info!("xxxx call api2 done. cost=80ms");
            }
        }
    }

    {
        // Spans will be sent to the configured OpenTelemetry exporter
        let root = span!(tracing::Level::TRACE, "app_exit", work_units = 3);
        let _enter = root.enter();
        std::thread::sleep(std::time::Duration::from_millis(60));
        info!("xxxx exit span. cost=60ms");
    }

    tracing::info!("begin call global::shutdown_tracer_provider");
    // Shutdown trace pipeline
    global::shutdown_tracer_provider();
    tracing::info!("end call global::shutdown_tracer_provider");
}
