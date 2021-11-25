# Rust tracing demo

https://github.com/open-telemetry/opentelemetry-rust

https://crates.io/crates/opentelemetry

common exporters:

opentelemetry-jaeger

opentelemetry-otlp

opentelemetry-prometheus

opentelemetry-zipkin


https://www.jaegertracing.io/docs/1.28/getting-started/#all-in-one

sudo podman run -d -p6831:6831/udp -p6832:6832/udp -p14268:14268 -p16686:16686 jaegertracing/all-in-one:latest

xdg-open http://localhost:16686/

