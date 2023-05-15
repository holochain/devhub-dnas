[back to README.md](README.md)


# Contributing

## Overview


### (high-level) Entity Relationship Diagram
![](https://drive.google.com/a/webheroes.ca/thumbnail?id=1bS9_PpZiwE5pA8mFTlRGvOYaWFwgzHPD&sz=w1000)

- DevHub
  - `dnarepo` DNA
    - DNA Repository Core
    - DNA Repository API
    - Reviews API
    - Mere Memory Core
    - Mere Memory API
  - `happs` DNA
    - hApps Core
    - hApps API
  - `web_assets` DNA
    - Web Assets Core
    - Web Assets API
    - Mere Memory Core
    - Mere Memory API

### Entity Relationship Pattern
This project implements the CEPS pattern *(pattern documentation is in process)*.

- [github.com/mjbrisebois/rust-hc-crud-ceps/](https://github.com/mjbrisebois/rust-hc-crud-ceps/)
- [docs.rs/hc_crud_ceps/](https://docs.rs/hc_crud_ceps/) - Rust library that implements th CEPS pattern

#### CEPS Concept Example
![](https://drive.google.com/a/webheroes.ca/thumbnail?sz=w1000&id=1oJTioA_IlGrcZI4tn-AyM0orMCt5kP6r)


## Development

### Naming Conventions

- Entry type names always end with `Entry`


### Environment

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


## Dependencies

- DevHub
  - CRUD (`hc_crud_ceps`)
  - Mere Memory
  - dev / testing
    - `@whi/holochain-client`
      - Holochain Conductor API
    - `@whi/holochain-backdrop`
      - Holochain Conductor API


#### [CRUD](https://crates.io/crates/hc_crud_ceps)

This library handles the underlying CRUD patter for all DevHub's entry types.  If there is a new HDK
for Holochain, the CRUD library will need to be bumped and possibly updated to handle any API
changes.


#### [Mere Memory](https://github.com/mjbrisebois/hc-zome-mere-memory)

Mere memory is a set of zomes used for simple byte storage.  Since it has an integrity and
coordinator zome, its version will change whenever the HDI or HDK are updated.


#### [`@whi/holochain-client`](https://github.com/mjbrisebois/js-holochain-client)

DevHub uses this for development and testing.  If the Conductor API has changed, this library will
most likely need to be updated.


#### [`@whi/holochain-backdrop`](https://github.com/mjbrisebois/node-holochain-backdrop)

DevHub uses this for development and testing.  This library is used to programmatically run the
Holochain binary, install hApps, creates agents, and make capability grants.  If the client
(`@whi/holochain-client`) API or Holochain manifests have changed, this library might need to be
updated.


### Upgrading for a new Holochain release

When there is a new Holochain release, we ony have to update affected parts.  This Causal Tree lists
the downstream DevHub components for parts of the Holochain engine.

#### Causal Tree

- HDI
  - DNA Repository Core
  - hApps Core
  - Web Assets Core
  - Mere Memory Core
- HDK
  - CRUD
  - DNA Repository API
  - Reviews API
  - hApps API
  - Web Assets API
  - Mere Memory API
- Conductor API
  - `@whi/holochain-client`
  - `@whi/holochain-backdrop`
- WebApp / App / DNA Manifest
  - `@whi/holochain-backdrop`
