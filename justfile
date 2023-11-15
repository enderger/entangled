build_web:
  trunk build

run_web:
  trunk serve

run_debug *ARGS:
  cargo run --features bevy/dynamic_linking -- {{ARGS}}

clean:
  rm -rf dist dist.tar.zstd

build:
  cargo build --release

dist_dir: clean build
  mkdir dist
  cp target/release/entangled dist/
  cp -r assets/ dist/

dist: dist_dir
  tar cvf dist.tar.zstd dist/
