# Proj4wkt

Convert WKT strings to proj4 strings.

Support both WKT1 and WKT2 formats.

This is a companion crate for `proj4rs`: because of this conversions are limited
to projection supported by `proj4rs`. As more projection will be supported in proj4rs, more conversions will be supported in `proj4wt.

Documentation on [doc.rs](https://docs.rs/proj4wkt/)

## Compiling for WASM

Install [wasm-pack](https://rustwasm.github.io/wasm-pack/book/)

```
wasm-pack build --target web --no-default-features
```

Or if you have installed [cargo-make](https://sagiegurari.github.io/cargo-make/), use the following
command:

```
cargo make wasm
```

### Running the WASM example

There is a [`index.html`] file for testing the WASM module in a navigator.

For security reason you need to run it from a server; you can pop up
a server from python with the following command:

```
python3 -m http.server
```

The server will automatically serve the `index.html` file in the current directory.
