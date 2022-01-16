# BudouX-rs Demo

Source code of BudouX-rs demo page.
The demo page is build with [Yew](https://github.com/yewstack/yew).

## Test

no tests.

<!--
```console
cargo test
```
-->

## Run Demo App

Install WebAssembly target if not added.

```console
rustup target add wasm32-unknown-unknown
```

Install [trunk](https://github.com/thedodd/trunk) and [wasm-bindgen-cli](https://github.com/rustwasm/wasm-bindgen) if not installed.

```console
cargo install trunk wasm-bindgen-cli
```

Run WebAssembly app.

```console
trunk serve
```

## Static Site Generate

Generate static website and output to dist directory.

```console
trunk build --release
```

If deploy to [GitHub Pages](https://pages.github.com/), set repository name to public url option.

```console
trunk build --release --public-url=REPOSITORY_NAME
```
