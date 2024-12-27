lib:
    cargo rustc --target wasm32-wasip1 -p game --crate-type=cdylib -F hotreload 
    wasm-tools component new ./target/wasm32-wasip1/debug/game.wasm -o target/debug/game.wasm --adapt vendor/wasi_snapshot_preview1.reactor.wasm

watch:
    cargo watch -C game -w ../wit -w . -- just lib 

run:
    cargo run -p launcher

hotreload:
    cargo run -p launcher --no-default-features -F hotreload 