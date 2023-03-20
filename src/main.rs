mod util;

use opentelemetry::{global, KeyValue, trace::Tracer};
use opentelemetry::sdk::{trace, Resource};
use opentelemetry_otlp::Protocol;
use opentelemetry_otlp::WithExportConfig;
use tracing::{trace_span, info_span, warn_span, error_span,  debug, info,  warn,error};
// use tracing_log::LogTracer;
use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter, Registry};

#[allow(unused_imports)]
use tracing_subscriber::prelude::*;

use std::time::Duration;
// use tonic::metadata::*;

#[tokio::main]
async fn main() {
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
    // let _jaeger_tracer = opentelemetry_jaeger::new_pipeline()
    //     .with_service_name("trace_demo")
    //     .with_collector_endpoint("http://localhost:14268/api/traces")
    //     .install_simple()
    //     .expect("Error initializing Jaeger exporter");

    let otlp_tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint("http://tempo.service.dc1.consul:4317")
                .with_protocol(Protocol::Grpc)
                .with_timeout(Duration::from_secs(3)),
        )
        .with_trace_config(trace::config()
                               .with_resource(Resource::new(vec![
                                   KeyValue::new("service.name", env!("CARGO_PKG_NAME")),
                                   KeyValue::new("service.version", env!("CARGO_PKG_VERSION")),
                               ])))
        // .install_simple()
        .install_batch(opentelemetry::runtime::Tokio)
        .expect("Error initializing Otlp exporter");

    // Create a tracing layer with the configured tracer
    let telemetry = tracing_opentelemetry::layer()
        // .with_tracer(stdout_tracer)
        // .with_tracer(jaeger_tracer);
        .with_tracer(otlp_tracer);

    // let format = fmt::format()
    //     .pretty()
    //     .with_source_location(true);


    // do not show tracing log message level < info
    let log_level_filter = tracing::level_filters::LevelFilter::from_level(tracing::Level::INFO);

    // warning: when using `LocalTime`, you'll need to build with `RUSTFLAGS="--cfg unsound_local_offset"` otherwise not tracing log will shown
    // see https://github.com/tokio-rs/tracing/pull/1699
    let fmt_layer = fmt::layer().with_timer(fmt::time::LocalTime::rfc_3339()).with_filter(log_level_filter);

    // Use the tracing subscriber `Registry`, or any other subscriber that impls `LookupSpan`
    // https://docs.rs/tracing-subscriber/0.3.3/tracing_subscriber/fmt/index.html#composing-layers
    // init() will set_global_default() and init tracing-log (also init default Logger)
    // see https://docs.rs/tracing-subscriber/0.3.3/tracing_subscriber/util/trait.SubscriberInitExt.html#method.init
    Registry::default().with(fmt_layer).with(telemetry).init(); /*.with(env_filter)*/

    // if let Err(err) = tracing::subscriber::set_global_default(subscriber) {
    //     panic!("setting tracing default subscriber failed, err={}", err)
    // }

    log::trace!("I: this is trace level log");
    log::info!("II: this is info level log");
    log::warn!("II: this is warn level log");
    log::error!("III: this is error level log");

    // Trace executed code
    // tracing::subscriber::with_default(subscriber, || {
    //     // Spans will be sent to the configured OpenTelemetry exporter
    //     let root = span!(tracing::Level::TRACE, "app_start", work_units = 2);
    //     let _enter = root.enter();
    //
    //     error!("xxxx go to root span.");
    // });

    do_work().await;

    tracing::info!("begin call global::shutdown_tracer_provider");
    // Shutdown trace pipeline
    global::shutdown_tracer_provider();
    tracing::info!("end call global::shutdown_tracer_provider");
}

#[tracing::instrument(name = "api::do_work")]
async fn do_work() {
    // Spans will be sent to the configured OpenTelemetry exporter
    // Levels are typically used to implement filtering that determines which spans and events are enabled. Depending on the use case, more or less verbose diagnostics may be desired.
    // For example, when running in development, DEBUG-level traces may be enabled by default.
    // When running in production, only INFO-level and lower traces might be enabled.
    // https://docs.rs/tracing/latest/tracing/struct.Level.html#filtering
    let root = trace_span!("do_work_start", work_units = 2);
    root.in_scope(||async{
            std::thread::sleep(std::time::Duration::from_millis(10));
            info!("xxxx start root span");
            warn_span!("api_call_1").in_scope(|| async{

                std::thread::sleep(std::time::Duration::from_millis(20));
                info!("xxxx call api1 done");

                // error_span does not trigger an error event, only for filter purposes
                // to trigger an error event, you need use tracing::error macro
                error_span!("api_call_2").in_scope(|| async{
                    std::thread::sleep(std::time::Duration::from_millis(30));
                    info!("xxxx call api2 done");

                    root.record("hello", &"world");

                    info_span!("api_call_3").in_scope(|| async{
                            std::thread::sleep(std::time::Duration::from_millis(40));
                            info!("xxxx call api3 done");
                            let user = find_by_username("ChristopherNolan").await;
                            info!("got user: {:?}", user);
                        }).await;

                }).await
            }).await
        }).await;
}


#[derive(Debug)]
struct UserInfo {
    username: String,
    display_name: String,
    email: String,
}

#[tracing::instrument(name = "model::find_by_username")]
async fn find_by_username(username: &str) -> UserInfo {
    debug!("sql: SELECT username, display_name, email FROM users WHERE username = {}", username);
    // trigger an error event, mark this call as failed
    error!("fake error: query user by name: {}", username);
    std::thread::sleep(std::time::Duration::from_millis(10));
    UserInfo{
        username: username.parse().unwrap(),
        display_name: "Christopher Nolan".to_string(),
        email: "hello@example.com".to_string()
    }
}
