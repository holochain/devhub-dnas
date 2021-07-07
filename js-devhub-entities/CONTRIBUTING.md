[back to README.md](README.md)

# Contributing

## Overview
This package is designed specifically for the Holochain DevHub project.  The entity model
definitions should match the structs defined in this hApp's [DNAs](../dnas/).

See [@whi/entity-architect](https://www.npmjs.com/package/@whi/entity-architect) for more
information about the Entity architecture.


## Development

### Environment

- Developed using Node.js `v12.20.0`

### Building
No build required.  Vanilla JS only.

### Testing

To run all tests with logging
```
make test-debug
```

- `make test-unit-debug` - **Unit tests only**
- `make test-integration-debug` - **Integration tests only**

> **NOTE:** remove `-debug` to run tests without logging
