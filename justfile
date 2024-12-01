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
@new DAY:
  if {{ path_exists(DAY) }}; then \
    echo 'folder already exists!'; \
    exit 1; \
  else \
    echo 'creating folder for day {{DAY}}!'; \
  fi
  cargo init --lib {{DAY}} --name day_{{DAY}}
  mkdir -p {{ join(DAY, 'src', 'part') }}
  touch {{ join(DAY, 'src', 'part', 'one.rs') }}
  touch {{ join(DAY, 'src', 'part', 'two.rs') }}
  touch {{ join(DAY, 'src', 'part', 'mod.rs') }}
  echo "mod one;\nmod two;\npub use one::*;\npub use two::*;" > {{ join(DAY, 'src', 'part', 'mod.rs') }}
  echo "mod part;\npub use part::*;" > {{ join(DAY, 'src', 'lib.rs') }}
  echo "Don't forget to update ci.yml ;)"
