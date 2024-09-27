# Find the target directory
CARGO_TARGET := `cargo metadata --format-version 1 --no-deps | jq -r '.target_directory'`
export ASPD_EXEC := CARGO_TARGET / "debug" / "aspd"
export BARK_EXEC := CARGO_TARGET / "debug" / "bark"

check-format:
	bash -c contrib/prechecks.sh rust_no_spaces_for_indent

build:
	cargo build --workspace

check:
  cargo check --all --tests

alias unit := test-unit
test-unit TEST="":
	cargo test --workspace --exclude ark-testing {{TEST}}

alias int := test-integration
test-integration TEST="": build
	cargo test --package ark-testing {{TEST}}

test: test-unit test-integration

# cleans most of our crates, doesn't clean grpc gens, they are sometimes slow to build
clean:
  cargo clean -p ark-lib -p ark-testing -p bark-aspd -p bark-client -p bark-json
