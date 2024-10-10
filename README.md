<div align="center">

  <h1><code>gml-wasm</code></h1>

  <strong>WebAssembly bindings for the gml-parser library <a href="https://github.com/zaporter/gml_parser"> gml_parser rust library </a>.</strong>

  <sub>Built with 🦀🕸</sub>
</div>

## About

This library is to enable Javascript applications to parse the Graph Modelling Language (GML) format used by a variety of different software packages. 

## 🚴 Usage


### 🛠️ Building

To build from source, you will need a rust toolchain and wasm-pack-build installed. More detailed instructions can be found [in the Rust WASM tutorial.](https://rustwasm.github.io/docs/book/game-of-life/setup.html)

Afterwards building is as simple as running:

```
wasm-pack build
```

### 🔬 Testing

Currently we are only testing in Chrome and Firefox.

Tests can be run with the following:

```
wasm-pack test --headless --firefox --chrome
```

### 🎁 Publish

To publish a new version, first log into npm using `wasm-pack adduser` and then publish using:

```
wasm-pack publish
```

## License

Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
