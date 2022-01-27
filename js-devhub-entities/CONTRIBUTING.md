[back to README.md](README.md)

# Contributing

## Overview
This package is designed specifically for the Holochain DevHub project.  The entity model
definitions should match the structs defined in this hApp's [zomes](../zomes/).

See [@whi/entity-architect](https://www.npmjs.com/package/@whi/entity-architect) for more
information about the Entity architecture.


## Development

### Environment

- Developed using Node.js `v14.17.3`

### Building
No build is required for Node.

Bundling with Webpack is supported for web
```
npm run build
```

#### Approximate size breakdown
Bundled size is `66kb`

- `5kb` - this package
- `5kb` - `@whi/essence`
- `16kb` - `@whi/entity-architect`
- `40kb` - `@whi/holochain-client`

### Testing

To run all tests with logging
```
make test-debug
```

- `make test-unit-debug` - **Unit tests only**
- `make test-integration-debug` - **Integration tests only**

> **NOTE:** remove `-debug` to run tests without logging
