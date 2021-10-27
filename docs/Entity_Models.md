[back to API.md](API.md)


# Entity Models

### Table of Contents

- Profile
  - Info
- Zome
  - Info
  - Summary
- Zome Version
  - Info
  - Summary
- DNA
  - Info
  - Summary
- DNA Version
  - Info
  - Summary
  - Package
- hApp
  - Info
  - Summary
- hApp Release
  - Info
  - Summary
- File
  - Info
  - Summary


## [`ProfileEntry`](Entry_Types.md#profile-profileentry)
Source [../devhub_types/src/dnarepo_entry_types.rs](../devhub_types/src/dnarepo_entry_types.rs)

### Profile Info `ProfileInfo`
No additional entries are fetched.

```javascript
{
    "name": String,
    "avatar_image": Uint8Array,

    // optional
    "email": String,
    "website": String,
}
```


## [`ZomeEntry`](Entry_Types.md#zome-zomeentry)
Source [../devhub_types/src/dnarepo_entry_types.rs](../devhub_types/src/dnarepo_entry_types.rs)

### Zome Info `ZomeInfo`
No additional entries are fetched.

```javascript
{
    "name": String,
    "description": String,
    "published_at": Number,
    "last_updated": Number,
    "developer": {
	"pubkey": Uint8Array(39), // AgentPubKey
    },

    // optional
    "deprecation": {
	"message": String,

        // optional
	"recommended_alternatives": [
	    Uint8Array(39), // EntryHash
	    ...
        ],
    }
}
```

### Zome Summary `ZomeSummary`

```javascript
{
    "name": String,
    "description": String,
    "published_at": Number,
    "last_updated": Number,
    "developer": Uint8Array(39), // AgentPubKey
    "deprecation": Boolean
}
```


## [`ZomeVersionEntry`](Entry_Types.md#zome-version-zomeversionentry)
Source [../devhub_types/src/dnarepo_entry_types.rs](../devhub_types/src/dnarepo_entry_types.rs)

### ZomeVersion Info `ZomeVersionInfo`
A `ZomeEntry` is fetched for this model.

> **NOTE:** `for_zome` is an `Option<...>` in the source code as a lazy way to handle errors when
> fetching.  This is not ideal and eventually needs to be done properly in a way that communicates
> any errors that occur.

```javascript
{
    "for_zome": ZomeSummary,
    "version": Number,
    "published_at": Number,
    "last_updated": Number,
    "changelog": String,
    "mere_memory_addr": Uint8Array(39), // EntryHash
}
```

### ZomeVersion Summary `ZomeVersionSummary`

```javascript
{
    "version": Number,
    "published_at": Number,
    "last_updated": Number,
    "mere_memory_addr": Uint8Array(39), // EntryHash
}
```


## [`DnaEntry`](Entry_Types.md#dna-dnaentry)
Source [../devhub_types/src/dnarepo_entry_types.rs](../devhub_types/src/dnarepo_entry_types.rs)

### DNA Info `DnaInfo`
No additional entries are fetched.

```javascript
{
    "name": String,
    "description": String,
    "published_at": Number,
    "last_updated": Number,
    "developer": {
        "pubkey": Uint8Array(39), // AgentPubKey
    },

    // optional
    "icon": Uint8Array,
    "deprecation": {
        "message": String,

        // optional
        "recommended_alternatives": [
            Uint8Array(39), // EntryHash
            ...
        ],
    }
}
```

### DNA Summary `DnaSummary`

```javascript
{
    "name": String,
    "description": String,
    "published_at": Number,
    "last_updated": Number,
    "developer": Uint8Array(39), // AgentPubKey
    "deprecation": Boolean

    // optional
    "icon": Uint8Array,
}
```


## [`DnaVersionEntry`](Entry_Types.md#dna-version-dnaversionentry)
Source [../devhub_types/src/dnarepo_entry_types.rs](../devhub_types/src/dnarepo_entry_types.rs)

### DNA Version Info `DnaVersionInfo`
A `DnaEntry` is fetched for this model.

> **NOTE:** `for_dna` is an `Option<...>` in the source code as a lazy way to handle errors when
> fetching.  This is not ideal and eventually needs to be done properly in a way that communicates
> any errors that occur.

```javascript
{
    "for_dna": DnaSummary,
    "version": Number,
    "published_at": Number,
    "last_updated": Number,
    "changelog": String,
    "zomes": [
        {
            "name": String,
            "zome": Uint8Array(39), // EntryHash
            "version": Uint8Array(39), // EntryHash
            "resource": Uint8Array(39), // EntryHash
        },
        ...
    ]
}
```

### DNA Version Summary `DnaVersionSummary`

```javascript
{
    "version": Number,
    "published_at": Number,
    "last_updated": Number,
    "zomes": [
        Uint8Array(39), // resource EntryHash
        ...
    ]
}
```

### DNA Version Package `DnaVersionPackage`

```javascript
{
    "for_dna": DnaSummary,
    "version": Number,
    "published_at": Number,
    "last_updated": Number,
    "changelog": String,
    "bytes": Uint8Array,
}
```


## [`HappEntry`](Entry_Types.md#happ-happentry)
Source [../devhub_types/src/happ_entry_types.rs](../devhub_types/src/happ_entry_types.rs)

### hApp Info `HappInfo`
No additional entries are fetched.

```javascript
{
    "title": String,
    "subtitle": String,
    "description": String,
    "designer": {
        "pubkey": Uint8Array(39), // AgentPubKey
    },
    "published_at": Number,
    "last_updated": Number,

    // optional
    "thumbnail_image": Uint8Array,
    "deprecation": {
        "message": String,

        // optional
        "recommended_alternatives": [
            Uint8Array(39), // EntryHash
            ...
        ],
    },
    "gui": {
        "asset_group_id": Uint8Array(39), // EntryHash
        "holo_hosting_settings": {
            "uses_web_sdk": Boolean
        }
    }
}
```

### hApp Summary `HappSummary`

```javascript
{
    "title": String,
    "subtitle": String,
    "description": String,
    "designer": Uint8Array(39), // AgentPubKey
    "published_at": Number,
    "last_updated": Number,
    "deprecation": Boolean,

    // optional
    "thumbnail_image": Uint8Array
}
```


## [`HappReleaseEntry`](Entry_Types.md#happ-release-happreleaseentry)
Source [../devhub_types/src/happ_entry_types.rs](../devhub_types/src/happ_entry_types.rs)

### hApp Release Info `HappReleaseInfo`
A `HappEntry` is fetched for this model.

> **NOTE:** `for_happ` is an `Option<...>` in the source code as a lazy way to handle errors when
> fetching.  This is not ideal and eventually needs to be done properly in a way that communicates
> any errors that occur.

```javascript
{
    "name": String,
    "description": String,
    "for_happ": HappSummary,
    "published_at": Number,
    "last_updated": Number,
    "manifest": {
        "manifest_version": String,
        "slots": [
            "id": String,
            "dna": {
                "bundled": String,
                "clone_limit": Number,

                // optional
                "uid": String,
                "version": String,
                "properties": { ... },
            },

            // optional
            "provisioning": {
                "strategy": String,
                "deferred": Boolean,
            }
        ],

        // optional
        "name": String,
        "description": String,
    },
    "dnas": [
        {
            "name": String,
            "zome": Uint8Array(39), // EntryHash
            "version": Uint8Array(39), // EntryHash
        },
        ...
    ]
}
```

### hApp Release Summary `HappReleaseSummary`

```javascript
{
    "name": String,
    "description": String,
    "for_happ": HappSummary,
    "published_at": Number,
    "last_updated": Number,
}
```


## [`FileEntry`](Entry_Types.md#file-fileentry)
Source [../devhub_types/src/web_asset_entry_types.rs](../devhub_types/src/web_asset_entry_types.rs)

### File Info `FileInfo`
No additional entries are fetched.

```javascript
{
    "author": Uint8Array(39), // AgentPubKey
    "published_at": Number,
    "file_size": Number,
    "chunk_addresses": [
        Uint8Array(39), // EntryHash
        ...
    ],
    "name": String
}
```

### File Summary `FileSummary`

```javascript
{
    "author": Uint8Array(39), // AgentPubKey
    "published_at": Number,
    "file_size": Number,
    "name": String
}
```
