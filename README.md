# Rust tracing demo

1. use format layer to display tracing as logs to stdout instead of a standalone trace pipline from `opentelemetry::sdk::export::trace::stdout`
2. `tracing_subscriber::fmt::init()` will set the global subscriber and global logger, in most cases you should avoid using it.
3. `env_logger::init()` will logging in **UTC** timezone, to use **localtime**, you need build your own `Logger`.
4. `tracing_subscriber::fmt::layer().with_timer(fmt::time::LocalTime::rfc_3339())` will not work 
and the log will **disappear** if you do not build with `RUSTFLAGS="--cfg unsound_local_offset"` 

https://docs.rs/tracing-subscriber/latest/tracing_subscriber/layer/index.html#runtime-configuration-with-layers

https://github.com/open-telemetry/opentelemetry-rust

https://crates.io/crates/opentelemetry

common exporters:

```
opentelemetry-jaeger

opentelemetry-otlp

opentelemetry-prometheus

opentelemetry-zipkin
```

https://www.jaegertracing.io/docs/1.28/getting-started/#all-in-one

```shell
sudo podman run --name jaeger -d -p6831:6831/udp -p6832:6832/udp -p14268:14268 -p16686:16686 jaegertracing/all-in-one:latest

xdg-open http://localhost:16686/
```



