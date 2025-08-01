all: release doc debug-linux

release: compile-release
	bash build.sh

compile-release: release-linux release-windows

release-linux:
	cargo build --release

release-windows:
	cargo build --target x86_64-pc-windows-gnu --release



debug-linux:
	cargo build

doc:
	cargo doc --document-private-items

start-server:
	twitch-cli event websocket start-server
