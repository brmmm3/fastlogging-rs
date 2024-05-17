# `jsfastlogging`

This is the WASM layer for [calclib](https://github.com/brmmm3/calc-rs/tree/master/calclib). This package is for creating a WASM extension using [wasm-pack](https://github.com/rustwasm/wasm-pack).

As a good starting point look at [rustwasm](https://rustwasm.github.io).

### 🛠️ Build with `wasm-pack build`

```
wasm-pack build
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```

### 🎁 Publish to NPM with `wasm-pack publish`

```
wasm-pack publish
```

## 🔋 Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
* `LICENSE-APACHE` and `LICENSE-MIT`: most Rust projects are licensed this way, so these are included for you
