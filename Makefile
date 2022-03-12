run:
	env RUST_LOG=debug RUSTFLAGS="--cfg unsound_local_offset" cargo run

trace:
	env RUST_LOG=trace RUSTFLAGS="--cfg unsound_local_offset" cargo run

ui:
	xdg-open http://localhost:16686/

jaeger:
	# https://www.jaegertracing.io/docs/1.32/getting-started/#prerequisites
	sudo podman run --name jaeger -d -p6831:6831/udp -p6832:6832/udp -p14268:14268 -p16686:16686 jaegertracing/all-in-one:latest

lint:
	cargo clippy

fmt:
	cargo fmt