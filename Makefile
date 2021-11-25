run:
	env RUST_LOG=debug cargo run

tracing:
	env RUST_LOG=tracing cargo run

ui:
	xdg-open http://localhost:16686/

jaeger:
	sudo podman run -d -p6831:6831/udp -p6832:6832/udp -p14268:14268 -p16686:16686 jaegertracing/all-in-one:latest

