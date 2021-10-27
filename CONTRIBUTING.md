[back to README.md](README.md)


# Contributing

## Overview


### (high-level) Entity Relationship Diagram
![](https://drive.google.com/a/webheroes.ca/thumbnail?id=1bS9_PpZiwE5pA8mFTlRGvOYaWFwgzHPD&sz=w1000)

### Entity Relationship Pattern
This project implements the CEPS pattern *(pattern documentation is in process)*.

- [github.com/mjbrisebois/rust-hc-crud-ceps/](https://github.com/mjbrisebois/rust-hc-crud-ceps/)
- [docs.rs/hc_crud_ceps/](https://docs.rs/hc_crud_ceps/) - Rust library that implements th CEPS pattern

#### CEPS Concept Example
![](https://drive.google.com/a/webheroes.ca/thumbnail?sz=w1000&id=1oJTioA_IlGrcZI4tn-AyM0orMCt5kP6r)


## Development

### Naming Conventions

- Entry type names always end with `Entry`
- Entity models ending in
  - `Info` - are representations of the corresponding `Entry` type, where properties may be
    reformatted or references to other entries may be replaced
    - eg. an `EntryHash` is fetched and replaced with the entry content
  - `Summary` - are a subset of properties from the corrosponding `Info` type.
    - *Summary types are intended to be quicker so they should avoid properties with content that
      would require additional DHT requests*


### Environment

- Developed using rustc `1.54.0 (a178d0322 2021-07-26)`
- Enter `nix-shell` for other development environment dependencies.


### Building

#### WASM Targets

```bash
make zomes/dna_library.wasm
make zomes/happ_library.wasm
make zomes/web_assets.wasm
```

Mere Memory WASM is not in this repo.  It must be build from
https://github.com/mjbrisebois/hc-zome-mere-memory and copied to `zomes/mere_memory.wasm`


#### DNA Targets

```bash
make bundled/dnarepo.dna
make bundled/happs.dna
make bundled/web_assets.dna
```


#### hApp Bundle

```bash
make DevHub.happ
```


### Testing

To run all tests with logging
```
make test-all-debug
```

- `make test-unit` - **Rust tests only**
- `make test-dnas-debug` - **DNA tests using Javascript**


> **NOTE:** remove `-debug` to run tests without logging
