[back to README.md](README.md)


# Contributing

## Overview


### (high-level) Entity Relationship Diagram
TODO


### Entity Relationship Pattern
This project implements the [CAPS
pattern](https://github.com/spartan-holochain-counsel/hc-4-pattern-vectors)



## Development

### Naming Conventions

- Entry type names always end with `Entry`


### Environment

- Enter `nix-shell` for other development environment dependencies.


### Building

#### hApp Bundle

```bash
make happ/devhub.happ
```


### Testing

To run all tests with logging
```
make test-debug
```

- `make test-unit-debug` - **Rust tests only**
- `make test-integration-debug` - **Integration tests only**

### Faux data tests

- `make test-webapp-upload-debug` - Upload faux webhapp

### Real long-running tests

- `make test-real-zome-upload-debug` - Upload `devhub.happ` to DevHub
- `make test-real-dna-upload-debug` - Upload `devhub.happ` to DevHub
- `make test-real-app-upload-debug` - Upload `devhub.happ` to DevHub


> **NOTE:** remove `-debug` to run tests without logging


## Dependencies

### Rust Dependency Tree

[![](https://mermaid.ink/img/pako:eNptlMtuwjAQRX8FeQUS_ACLSlaMZNRdnW6SoMiNTYMgD-XRliL-vaZ4xjDAintm5urOJHBiRWMsW7LtofkuSt0Nk1hk9cR9pFinpdltUK0uMrc_g637XVP3vpLE6W9T2bwcP_Lh2FrgIk5NrR8wj1Pdtg84UTc2fdG01ty7qWD3pMxVsH1STuQUzGfeT069nQdcTr3BDGbENEzhmIMwiJOOwSz2yTQ19suhjctwhdHbu0jLIi-60eSFbnu87as77T6oy6X3j5cWUnnPvMf2RN3cDalQ4VoIuQo3CvMywi3zou9wgQj2vKHcUbC4Unw7JovFyyU7BesVrgUtgaz-kdvrxujKkjjc7ApUDO8b0coDCQt5GZG6wive6USRfpgXeKAASEpBUwpIJWKifQohIYWXEamTlKgV6Yd5gc8rAJKS05QcUvGYaJ-CS3jeXkakTlKC5or0w7zA9ycA_Jn54HgZAngAbM4q21V6Z9xf1ulSzthQ2spmbOm-GrvV42HIWFafXaseh0Yd64Ith260cza2Rg9W7PRnpyu23OpDb89_f4tpYw?type=png)](https://mermaid.live/edit#pako:eNptlMtuwjAQRX8FeQUS_ACLSlaMZNRdnW6SoMiNTYMgD-XRliL-vaZ4xjDAintm5urOJHBiRWMsW7LtofkuSt0Nk1hk9cR9pFinpdltUK0uMrc_g637XVP3vpLE6W9T2bwcP_Lh2FrgIk5NrR8wj1Pdtg84UTc2fdG01ty7qWD3pMxVsH1STuQUzGfeT069nQdcTr3BDGbENEzhmIMwiJOOwSz2yTQ19suhjctwhdHbu0jLIi-60eSFbnu87as77T6oy6X3j5cWUnnPvMf2RN3cDalQ4VoIuQo3CvMywi3zou9wgQj2vKHcUbC4Unw7JovFyyU7BesVrgUtgaz-kdvrxujKkjjc7ApUDO8b0coDCQt5GZG6wive6USRfpgXeKAASEpBUwpIJWKifQohIYWXEamTlKgV6Yd5gc8rAJKS05QcUvGYaJ-CS3jeXkakTlKC5or0w7zA9ycA_Jn54HgZAngAbM4q21V6Z9xf1ulSzthQ2spmbOm-GrvV42HIWFafXaseh0Yd64Ith260cza2Rg9W7PRnpyu23OpDb89_f4tpYw)

### External Depependencies

- CRUD ([`hc_crud_caps`](https://docs.rs/hc_crud_caps))
- Mere Memory ([`mere_memory_types`](https://docs.rs/mere_memory_types))

#### Dev / Testing Dependencies

- [`@spartan-hc/app-interface-client`](https://github.com/spartan-holochain-counsel/app-interface-client-js)
- [`@spartan-hc/holochain-backdrop`](https://github.com/spartan-holochain-counsel/node-holochain-backdrop)


#### CRUD ([github.com/spartan-holochain-counsel/rust-hc-crud-caps](https://github.com/spartan-holochain-counsel/rust-hc-crud-caps))

This library handles the underlying patterns for types that require CRUD.


#### Mere Memory ([github.com/spartan-holochain-counsel/zome-mere-memory](https://github.com/spartan-holochain-counsel/zome-mere-memory))

Mere memory is a set of zomes used for simple byte storage.


#### `@spartan-hc/app-interface-client`

DevHub uses this for development and testing.  This library changes as the Conductor's App Interface
API evolves.


#### `@spartan-hc/holochain-backdrop`

DevHub uses this for development and testing.  This library is used to programmatically run the
Holochain binary, install hApps, creates agents, and make capability grants.  This library changes
as the Conductor's API evolves.
