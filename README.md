Rust Lander

A game project excersise using Bevy and Rust.

# Web Target

easy setup
```aiignore
cargo run --target wasm32-unknown-unknown --no-default-features
```

use the wasm32 target
```aiignore
rustup target add wasm32-unknown-unknown
```


use the bindgen tool
```aiignore
cargo install wasm-bindgen-cli
```

build for relese.
*note*: dynamic-linking will have to be disabled
```aiignore
cargo build --release --target wasm32-unknown-unknown

wasm-bindgen --out-dir ./dist --target web ./target/wasm32-unknown-unknown/release/your_game_name.wasm
```




# Desktokp Target