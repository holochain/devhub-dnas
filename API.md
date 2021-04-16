[back to README.md](README.md)

# API Reference

## Overview

### Naming Conventions

- Entry type names always end with `Entry`
- Response types ending in
  - `Info` - usually have a corrosponding entry type, but the `Info` structure has replaced
    references (eg. `EntryHash`) with the content.
  - `Summary` - are a subset of properties from the corrosponding `Info` type.
    - *Summary types are intended to be quicker so they should avoid properties with content that
      would require additional DHT requests*


### List of Actions

#### `viewer`

- Keyword search for hApps
- Get hApp (metadata)
- Get latest manifest
- Get DNA versions
- Get DNA package

Support for remote calling

- Keyword search for hApps
- Get hApp (metadata)
- Get latest manifest
- Get DNA versions
- Get DNA package


#### `happs`

Reads
- Get hApp (metadata)
- Get latest manifest
- Get my hApps
- Get hApp versions for hApp

Writes
- Create hApp
- Update hApp
- Delete hApp
- Create hApp version
- Update hApp version
- Delete hApp version


#### `happs_index`

- Keyword search


#### `dnas`

Reads
- Get DNA package
- Get my DNAs (metadata)
- Get DNA versions for DNA (metadata)

Writes
- Create DNA (metadata)
- Update DNA (metadata)
- Deprecate DNA (metadata)
- Create DNA version
- Update DNA version
- Delete DNA version



## `viewer` DNA Zome Functions

### `happ` Zome

#### `search( args )`
Keyword search for hApps


#### `get( { addr: EntryHash } ) -> AppInfo`
Get hApp (metadata)


#### `get_manifest( { addr: EntryHash } ) -> ManifestInfo`
Get latest manifest


### `dna` Zome

#### `get_versions_for_dna( { addr: EntryHash } ) -> Vec<DnaVersionSummary>`
Get DNA versions


#### `get_package( { addr: EntryHash } ) -> DnaPackage`
Get DNA package



## `happs` DNA Zome Functions

### `store` Zome

#### `get_app( { addr: EntryHash } ) -> AppInfo`
Get hApp (metadata)


#### `get_manifest( { addr: EntryHash } ) -> ManifestInfo`
Get latest manifest


#### `get_my_apps() -> Vec<AppSummary>`
Get my hApps


#### `get_manifests_for_app( { addr: EntryHash } ) -> Vec<ManifestSummary>`
Get hApp versions for hApp


#### `create_app( args ) -> (EntryHash, AppInfo)`
Create hApp


#### `update_app( { addr: EntryHash, ...args } ) -> (EntryHash, AppInfo)`
Update hApp


#### `delete_app( { addr: EntryHash } ) -> HeaderHash`
Delete hApp


#### `create_manifest( args ) -> (EntryHash, ManifestInfo)`
Create hApp version


#### `update_manifest( { addr: EntryHash, ...args } ) -> (EntryHash, ManifestInfo)`
Update hApp version


#### `delete_manifest( { addr: EntryHash } ) -> HeaderHash`
Delete hApp version



## `happs_index` DNA Zome Functions

### `keyword` Zome

#### `search( { keywords: String } ) -> Vec<AppSummary>`
Keyword search



## `dnas` DNA Zome Functions

### `storage` Zome

#### `get_package( { addr: EntryHash } ) -> DnaPackage`
Get DNA package


#### `get_dna( { addr: EntryHash } ) -> DnaInfo`
Get my DNAs (metadata)


#### `get_dna_version( { addr: EntryHash } ) -> DnaVersionInfo`
Get DNA versions for DNA (metadata)


#### `create_dna( { addr: EntryHash } ) -> (EntryHash, DnaInfo)`
Create DNA (metadata)


#### `update_dna( { addr: EntryHash, ...args } ) -> (EntryHash, DnaInfo)`
Update DNA (metadata)


#### `deprecate_dna( { addr: EntryHash, ...args } ) -> (EntryHash, DnaInfo)`
Deprecate DNA (metadata)


#### `create_dna_version( args ) -> (EntryHash, DnaVersionInfo)`
Create DNA version


#### `update_dna_version( { addr: EntryHash, ...args } ) -> (EntryHash, DnaVersionInfo)`
Update DNA version


#### `delete_dna_version( { addr: EntryHash } ) -> HeaderHash`
Delete DNA version



## Entry Types

### hApp `AppEntry`

```javascript
{
    "title": String,
    "subtitle": String,
    "description": String,
    "thumbnail_image": EntryHash,
    "designer": AgentPubKey,
    "maintained_by": {
        "name": String,
        "website": String,
    },
    "categories": [
        EntryHash,
    ],
}
```


### hApp Version `ManifestEntry`

```javascript
{
    "name": String,
    "description": String,
    "for_happ": EntryHash,
    "manifest_version": Integer,
    "cells": [{
        "nick": String,
        "provisioning": ?{
            "strategy": String,
            "deferred": Boolean,
        },
        "dna": {
            "entry_id": Array(32),
            "url": ?String,
            "uuid": ?String,
            "overridable": Boolean,
            "version": ?Array<Array(32)>,
            "clone_limit": ?Integer,
            "properties": ?,
        },
    }],
}
```

### DNA `DnaEntry`

```javascript
{
    "name": String,
    "description": String,
    "developer": {
        "name": String,
        "website": String,
    },
    "deprecation": ?{
        "message": String,
        "recommended_alternatives": [
            EntryHash,
        ]
    }
}
```

### DNA Version `DnaVersionEntry`

```javascript
{
    "for_dna": EntryHash,
    "version": Integer,
    "published_at": Time,
    "file_size": Integer,
    "contributors": [
        String,
        ...
    ],
    "changelog": String,
    "chunk_addresses": [
        EntryHash,
        ...
    ]
}
```

### DNA Chunk `DnaChunkEntry`

```javascript
{
    "sequence": {
        "position": Integer,
        "length": Integer,
    },
    "bytes": SerializedBytes,
}
```

## Response Structs

Standard
- Entry hash
- Header hash

hApps
- hApp summary
- hApp info
- Manifest summary
- Manifest info

DNAs
- DNA summary
- DNA info
- DNA version summary
- DNA version info
- DNA package (with chunks)


### hApp Summary `AppSummary`

```javascript
{
    "title": String,
    "subtitle": String,
    "thumbnail_image": SerializedBytes,
    "designer": AgentPubKey,
    "categories": [
        String,
        ...
    ],
}
```


### hApp Info `AppInfo`

```javascript
{
    "title": String,
    "subtitle": String,
    "description": String,
    "thumbnail_image": SerializedBytes,
    "designer": AgentPubKey,
    "maintained_by": {
        "name": String,
        "website": String,
    },
    "categories": [
        String,
        ...
    ],
}
```


### Manifest Summary `ManifestSummary`


```javascript
{
    "name": String,
    "description": String,
    "manifest_version": Integer,
    "cells": [{
        "nick": String,
        "dna": {
            "entry_id": Array(32),
        },
    }],
}
```


### Manifest Info `ManifestInfo`

```javascript
{
    "name": String,
    "description": String,
    "for_happ": AppSummary,
    "manifest_version": Integer,
    "cells": [{
        "nick": String,
        "provisioning": ?{
            "strategy": String,
            "deferred": Boolean,
        },
        "dna": {
            "entry_id": Array(32),
            "url": ?String,
            "uuid": ?String,
            "overridable": Boolean,
            "version": ?Array<Array(32)>,
            "clone_limit": ?Integer,
            "properties": ?,
        },
    }],
}
```


### DNA Summary `DnaSummary`

```javascript
{
    "name": String,
    "description": String,
    "developer": String,
    "deprecation": Boolean,
}
```


### DNA Info `DnaInfo`

```javascript
{
    "name": String,
    "description": String,
    "developer": {
        "name": String,
        "website": String,
    },
    "deprecation": ?{
        "message": String,
        "recommended_alternatives": ?[
            DnaSummary,
        ]
    }
}
```


### DNA Version Summary `DnaVersionSummary`

```javascript
{
    "version": Integer,
    "published_at": Time,
    "file_size": Integer,
}
```


### DNA Version Info `DnaVersionInfo`

```javascript
{
    "for_dna": DnaSummary,
    "version": Integer,
    "published_at": Time,
    "file_size": Integer,
    "contributors": [
        String,
        ...
    ],
    "changelog": String,
    "chunk_addresses": [
        EntryHash,
        ...
    ]
}
```


### DNA Package `DnaPackage`

```javascript
{
    "for_dna": DnaSummary,
    "version": Integer,
    "published_at": Time,
    "file_size": Integer,
    "bytes": SerializedBytes,
    "contributors": [
        String,
        ...
    ],
    "changelog": String,
}
```


### Error `WasmError`

```javascript
?
```
