build::
	rm -rf cureq cureq.h
	cargo build
	cbindgen --lang c --output cureq.h
	cc cureq.c -L./target/debug -lcureq -o cureq

run:: build
	./cureq

watch::
	git ls-files | entr -rc make run

release::
	cargo build --release --target x86_64-unknown-linux-musl
	cbindgen --lang c --output cureq.h
	cp ./target/x86_64-unknown-linux-musl/release/libcureq.a .
	cc cureq.c --static -L./target/x86_64-unknown-linux-musl/release -lcureq -o cureq