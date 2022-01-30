http:
	cargo run --bin httpserver
ws:
	cargo run --bin wsserver
build:
	export PATH="$HOME/tools/x86_64-unknown-linux-musl/bin:$PATH"
	export CC_x86_64_unknown_linux_musl='x86_64-unknown-linux-musl-gcc'
	export CXX_x86_64_unknown_linux_musl='x86_64-unknown-linux-musl-g++'
	export AR_x86_64_unknown_linux_musl='x86_64-unknown-linux-musl-ar'
	export CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER='x86_64-unknown-linux-musl-gcc'
	cargo build --target x86_64-unknown-linux-musl --release --bin httpserver
	cargo build --target x86_64-unknown-linux-musl --release --bin wsserver
zip: 
	rm -rf hs.zip
	rm -rf dst
	mkdir dst
	cp ./target/x86_64-unknown-linux-musl/release/httpserver ./dst/
	cp ./target/x86_64-unknown-linux-musl/release/wsserver ./dst/
	cp -r ./assets ./dst/assets
	cp -r ./nginx.conf ./dst/
	zip -r hs.zip ./dst	