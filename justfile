run DAY:
  cargo run --package day_{{DAY}}
test DAY:
  cargo test --package day_{{DAY}} --lib
bench DAY:
  cargo bench --package day_{{DAY}}
qa:
  cargo clippy -- -D warnings
  cargo fix
  cargo fmt --check
lint:
  cargo clippy --fix --allow-dirty --allow-staged --all-features
  cargo fix --allow-dirty --allow-staged --all-features
  cargo fmt --all