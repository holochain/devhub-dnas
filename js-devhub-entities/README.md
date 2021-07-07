[![](https://img.shields.io/npm/v/@holochain/devhub-entities/latest?style=flat-square)](http://npmjs.com/package/@holochain/devhub-entities)

# DevHub Entity Architecture
A Javascript library for deconstructing DevHub response payloads.


## Overview
This package defines Entity models for the Holochain DevHub project using the
`@whi/entity-architect` library.


## Install

```bash
npm i @holochain/devhub-entities
```

## Basic usage

```javascript
const { Schema } = require('@holochain/devhub-entities');

const ID = Buffer.from("hCEkEvFsj08QdtgiUDBlEhwlcW5lsfqD4vKRcaGIirSBx0Wl7MVf", "base64");
const HEADER = Buffer.from("hCkkn_kIobHe9Zt4feh751we8mDGyJuBXR50X5LBqtcSuGLalIBa", "base64");
const ADDRESS = Buffer.from("hCEkU7zcM5NFGXIljSHjJS3mk62FfVRpniZQlg6f92zWHkOZpb2z", "base64");
const AUTHOR = Buffer.from("hCAkocJKdTlSkQFVmjPW_lA_A5kusNOORPrFYJqT8134Pag45Vjf", "base64");

Schema.deconstruct( "entity", {
    "id": ID,
    "header": HEADER,
    "address": ADDRESS,
    "type": {
        "name": "dna",
        "model": "info",
    },
    "content": dna_info
});
// Entity { ...dna_info }
```

### Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md)
