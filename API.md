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
- Create DNA chunk



## `viewer` DNA Zome Functions

### `happ` Zome

#### `search( { keywords: String, ...options } ) -> Vec<AppSummary>`
Keyword search for hApps

```javascript
let apps = client.call("viewer", "happ", "search", {
    "keywords": "spider solitaire",
});
```


#### `get_happ( { addr: EntryHash } ) -> AppInfo`
Get hApp (metadata)

```javascript
let app = client.call("viewer", "happ", "get_happ", {
    "addr": <Buffer 84 21 24 ...>,
});
```


#### `get_happ_manifest( { addr: EntryHash } ) -> ManifestInfo`
Get latest manifest

```javascript
let manifest = client.call("viewer", "happ", "get_happ_manifest", {
    "addr": <Buffer 84 21 24 ...>,
});
```


### `dna` Zome

#### `get_dna_versions( { for_dna: EntryHash } ) -> Vec<DnaVersionSummary>`
Get DNA versions

```javascript
let versions = client.call("viewer", "happ", "get_dna_versions", {
    "for_dna": <Buffer 84 21 24 ...>,
});
```


#### `get_dna_package( { addr: EntryHash } ) -> DnaPackage`
Get DNA package

```javascript
let dna = client.call("viewer", "happ", "get_dna_package", {
    "addr": <Buffer 84 21 24 ...>,
});
```



## `happs` DNA Zome Functions

#### Entity Relationship Diagram
![](https://drive.google.com/a/webheroes.ca/thumbnail?id=1amiyBBUt2JAPz1PhknOv3wFg6v-tKx5I&sz=w1000)


### `store` Zome

#### `get_app( { addr: EntryHash } ) -> AppInfo`
Get hApp (metadata)

```javascript
let happ = client.call("happs", "store", "get_app", {
    "addr": <Buffer 84 21 24 ...>,
});
```


#### `get_manifest( { addr: EntryHash } ) -> ManifestInfo`
Get latest manifest

```javascript
let manifest = client.call("happs", "store", "get_manifest", {
    "addr": <Buffer 84 21 24 ...>,
});
```


#### `get_my_apps() -> Vec<AppSummary>`
Get my hApps

```javascript
let happs = client.call("happs", "store", "get_my_apps", null);
```


#### `get_manifests( { for_happ: EntryHash } ) -> Vec<ManifestSummary>`
Get hApp versions for hApp

```javascript
let manifests = client.call("happs", "store", "get_manifests", {
    "for_happ": <Buffer 84 21 24 ...>,
});
```


#### `create_app( input ) -> (EntryHash, AppInfo)`
Create hApp

```javascript
let [happ_hash, happ] = client.call("happs", "store", "create_app", {
    "title": "Spider Solitaire",
    "subtitle": "The popular classic card game",
    "description": "Play the #1 classic Spider Solitaire for Free! ...",
    "thumbnail_image": Uint8Array(10392) [121, 111, 117,  32,  99,  97, 110,  39, ...],
    "maintained_by": {
        "name": "Open Games Collective",
        "website": "https://open-games.example",
    },
    "categories": [ "Card Games", "Games" ],
});
```


#### `update_app( { addr: EntryHash, properties: { ...updates } } ) -> (EntryHash, AppInfo)`
Update hApp

```javascript
let [updated_happ_hash, happ] = client.call("happs", "store", "update_app", {
    "addr": happ_hash,
    "properties": {
        "subtitle": "The popular classic card game",
        "description": "Play the #1 classic Spider Solitaire for Free! ...",
    },
});
```


#### `delete_app( { addr: EntryHash } ) -> HeaderHash`
Delete hApp

```javascript
let delete_happ_hash = client.call("happs", "store", "delete_app", {
    "addr": happ_hash,
});
```


#### `create_manifest( input ) -> (EntryHash, ManifestInfo)`
Create hApp version

```javascript
let [manifest_hash, manifest] = client.call("happs", "store", "create_manifest", {
    "for_happ": happ_hash,
    "name": "Beta 1.0",
    "description": "First beta release",
    "manifest_version": 1,
    "cells": [{
        "nick": "game-turns",
        "dna": {
            "entry_id": dna_hash,
            "uuid": "52c2148c-39ca-47d0-a391-045172c3e09b",
            "overridable": false,
            "clone_limit": 1,
        },
    }],
});
```


#### `update_manifest( { addr: EntryHash, properties: { ...updates } } ) -> (EntryHash, ManifestInfo)`
Update hApp version

```javascript
let [updated_manifest_hash, manifest] = client.call("happs", "store", "update_manifest", {
    "addr": manifest_hash,
    "properties": {
        "cells": [{
            "dna": {
                "url": "https://github.com/open-games-collective/game-turns/",
                "version": [
                    dna_version_hash,
                ],
            },
        }],
    },
});
```


#### `delete_manifest( { addr: EntryHash } ) -> HeaderHash`
Delete hApp version

```javascript
let delete_manifest_hash = client.call("happs", "store", "delete_manifest", {
    "addr": manifest_hash,
});
```



## `happs_index` DNA Zome Functions

### `search` Zome

#### `keywords( { phrase: String } ) -> Vec<AppSummary>`
Keyword search

```javascript
let apps = client.call("happs_index", "search", "keywords", {
    "phrase": "spider solitaire",
});
```



## `dnarepo` DNA Zome Functions

#### Entity Relationship Diagram
![](https://drive.google.com/a/webheroes.ca/thumbnail?id=1VjYZPu0A4-yjhH53Fefr1FEWNdQdgCJ4&sz=w1000)

### `storage` Zome

#### `get_package( { addr: EntryHash } ) -> DnaPackage`
Get DNA package

```javascript
let dna = client.call("dnarepo", "dna_library", "get_package", {
    "addr": <Buffer 84 21 24 ...>,
});
```


#### `get_dna( { addr: EntryHash } ) -> DnaInfo`
Get a DNA (metadata)

```javascript
let dna = client.call("dnarepo", "dna_library", "get_dna", {
    "addr": dna_hash,
});
```


#### `get_my_dnas() -> Vec<DnaSummary>`
Get my DNA's (metadata)

```javascript
let dnas = client.call("dnarepo", "dna_library", "get_my_dnas", null );
```


#### `get_dna_versions( { for_dna: EntryHash } ) -> Vec<DnaVersionSummary>`
Get DNA versions for DNA (metadata)

```javascript
let dna_versions = client.call("dnarepo", "dna_library", "get_dna_versions", {
    "for_dna": <Buffer 84 21 24 ...>,
});
```


#### `create_dna( { addr: EntryHash } ) -> (EntryHash, DnaInfo)`
Create DNA (metadata)

```javascript
let [dna_hash, dna_info] = client.call("dnarepo", "dna_library", "create_dna", {
    "name": "Game Turns",
    "description": "A tool for turn-based games to track the order of player actions",
});
```


#### `update_dna( { addr: EntryHash, properties: { ...updates } } ) -> (EntryHash, DnaInfo)`
Update DNA (metadata)

```javascript
let [updated_dna_hash, dna_info] = client.call("dnarepo", "dna_library", "update_dna", {
    "addr": dna_hash,
    "properties": {
        "developer": {
            "name": "Open Games Collective",
            "website": "https://github.com/open-games-collective/",
        }
    }
});
```


#### `deprecate_dna( { addr: EntryHash, ...options } ) -> (EntryHash, DnaInfo)`
Deprecate DNA (metadata)

```javascript
let [deprecated_dna_hash, dna_info] = client.call("dnarepo", "dna_library", "deprecate_dna", {
    "addr": dna_hash,
    "properties": {
        "deprecation": {
            "message": "No longer maintained",
        }
    }
});
```


#### `create_dna_version( input ) -> (EntryHash, DnaVersionInfo)`
Create DNA version

```javascript
let [dna_version_hash, dna_info] = client.call("dnarepo", "dna_library", "create_dna_version", {
    "for_dna": dna_hash,
    "version": 1,
    "published_at": 1618855430,
    "file_size": 739038,
    "chunk_addresses": [
        dna_chunk1_hash,
        dna_chunk2_hash,
        ...
        dna_chunk73_hash,
    ]
});
```


#### `update_dna_version( { addr: EntryHash, properties: { ...updates } } ) -> (EntryHash, DnaVersionInfo)`
Update DNA version

```javascript
let [updated_dna_version_hash, dna_info] = client.call("dnarepo", "dna_library", "create_dna_version", {
    "addr": dna_version_hash,
    "properties": {
        "changelog": "# Changelog\nFeatures\n...",
        "contributors": [
            "kevin@open-games.example",
            "stuart@open-games.example",
            "bob@open-games.example",
        ],
    }
});
```


#### `delete_dna_version( { addr: EntryHash } ) -> HeaderHash`
Delete DNA version

```javascript
let delete_dna_version_hash = client.call("dnarepo", "dna_library", "delete_dna_version", {
    "addr": dna_version_hash,
});
```


#### `create_dna_chunk( { sequence: {...}, bytes: SerializedBytes } ) -> EntryHash`
Create DNA version

```javascript
let dna_chunk1_hash = client.call("dnarepo", "dna_library", "create_dna_chunk", {
    "sequence": {
        "position": 1,
        "length": 73,
    },
    "bytes": Uint8Array(10240) [36, 83, 132, 33, 27, 192, 10, 137, ...],
});
```



## Entry Types

### hApp `AppEntry`

```javascript
{
    "title": String,
    "subtitle": String,
    "description": String,
    "thumbnail_image": EntryHash,
    "published_at": Time,
    "architect": AgentPubKey,
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
    "for_happ": EntryHash,
    "name": String,
    "description": String,
    "manifest_version": Integer,
    "published_at": Time,
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
    "published_at": Time,
    "developer": ?{
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
    "bytes": SerializedBytes, // max length 10,485,760
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

DNArepo
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
    "published_at": Time,
    "architect": AgentPubKey,
    "categories": [
        String,
        ...
    ],
}
```

#### Example
```javascript
{
    "title": "Spider Solitaire",
    "subtitle": "The popular classic card game",
    "thumbnail_image": Uint8Array(10392) [121, 111, 117,  32,  99,  97, 110,  39, ...],
    "published_at": 1618855430,
    "architect": Uint8Array(39) [132, 32, 36, 246, 137, 10, 220, 57, ...],
    "categories": [ "Card Games", "Games" ],
}
```


### hApp Info `AppInfo`

```javascript
{
    "title": String,
    "subtitle": String,
    "description": String,
    "thumbnail_image": SerializedBytes,
    "published_at": Time,
    "architect": AgentPubKey,
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

#### Example
```javascript
{
    "title": "Spider Solitaire",
    "subtitle": "The popular classic card game",
    "description": "Play the #1 classic Spider Solitaire for Free! ...xo",
    "thumbnail_image": Uint8Array(10392) [121, 111, 117,  32,  99,  97, 110,  39, ...],
    "published_at": 1618855430,
    "architect": Uint8Array(39) [132, 32, 36, 246, 137, 10, 220, 57, ...],
    "maintained_by": {
        "name": "Open Games Collective",
        "website": "https://open-games.example",
    },
    "categories": [ "Card Games", "Games" ],
}
```


### Manifest Summary `ManifestSummary`

```javascript
{
    "name": String,
    "description": String,
    "manifest_version": Integer,
    "published_at": Time,
    "cells": [{
        "nick": String,
        "dna": {
            "entry_id": Array(39),
        },
    }],
}
```

#### Example
```javascript
{
    "name": "Beta 1.0",
    "description": "First beta release",
    "manifest_version": 1,
    "published_at": 1618855430,
    "cells": [{
        "nick": "game-turns",
        "dna": {
            "entry_id": Uint8Array(39) [132, 33, 36, 246, 137, 10, 220, 57, ...],
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
    "published_at": Time,
    "cells": [{
        "nick": String,
        "provisioning": ?{
            "strategy": String,
            "deferred": Boolean,
        },
        "dna": {
            "entry_id": Array(39),
            "url": ?String,
            "uuid": ?String,
            "overridable": Boolean,
            "version": ?Array<Array(39)>,
            "clone_limit": ?Integer,
            "properties": ?,
        },
    }],
}
```

#### Example
```javascript
{
    "name": "Beta 1.0",
    "description": "First beta release",
    "manifest_version": 1,
    "published_at": 1618855430,
    "cells": [{
        "nick": "game-turns",
        "dna": {
            "entry_id": Uint8Array(39) [132, 33, 36, 246, 137, 10, 220, 57, ...],
            "url": "https://github.com/open-games-collective/game-turns/",
            "uuid": "52c2148c-39ca-47d0-a391-045172c3e09b",
            "overridable": false,
            "version": [
                Uint8Array(39) [132, 33, 36, 10, 220, 57, 246, 137, ...],
            ],
            "clone_limit": 1,
        },
    }],
}
```


### DNA Summary `DnaSummary`

```javascript
{
    "name": String,
    "description": String,
    "published_at": Time,
    "developer": ?String,
    "deprecation": ?Boolean,
}
```

#### Example
```javascript
{
    "name": "Game Turns",
    "description": "A tool for turn-based games to track the order of player actions",
    "published_at": 1618855430,
    "developer": "Open Games Collective",
}
```


### DNA Info `DnaInfo`

```javascript
{
    "name": String,
    "description": String,
    "published_at": Time,
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

#### Example
```javascript
{
    "name": "Game Turns",
    "description": "A tool for turn-based games to track the order of player actions",
    "published_at": 1618855430,
    "developer": {
        "name": "Open Games Collective",
        "website": "https://github.com/open-games-collective/",
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

#### Example
```javascript
{
    "version": 1,
    "published_at": 1618855430,
    "file_size": 739038,
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

#### Example
```javascript
{
    "for_dna": Uint8Array(39) [132, 33, 36, 10, 220, 57, 246, 137, ...],
    "version": 1,
    "published_at": 1618855430,
    "file_size": 739038,
    "contributors": [
        "kevin@open-games.example",
        "stuart@open-games.example",
        "bob@open-games.example",
    ],
    "changelog": "# Changelog\nFeatures\n...",
    "chunk_addresses": [
        Uint8Array(39) [132, 33, 36, 83, 27, 192, 10, 137, ...],
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

#### Example
```javascript
{
    "for_dna": Uint8Array(39) [132, 33, 36, 10, 220, 57, 246, 137, ...],
    "version": 1,
    "published_at": 1618855430,
    "file_size": 739038,
    "bytes": Uint8Array(39) [105, 101,  57, 108, 97, 101, 110,  98, ...],
    "contributors": [
        "kevin@open-games.example",
        "stuart@open-games.example",
        "bob@open-games.example",
    ],
    "changelog": "# Changelog\nFeatures\n...",
}
```


### Error `WasmError`

```javascript
?
```
