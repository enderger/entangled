build_web:
  trunk build

run_web:
  trunk serve

run_debug *ARGS:
  cargo run --features bevy/dynamic_linking -- {{ARGS}}

clean:
  rm -r dist

build:
  cargo build --release

dist: clean build
  mkdir dist
  cp target/release/entangled dist/
  cp -r assets/ dist/
  tar cvf dist.tar.zstd dist/
