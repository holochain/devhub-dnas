[back to API.md](../API.md)


# `dna_library` - Zome API Reference

## Entry Types Used

- [`ProfileEntry`](../Entry_Types.md#profile-profileentry)
- [`ZomeEntry`](../Entry_Types.md#zome-zomeentry)
- [`ZomeVersionEntry`](../Entry_Types.md#zome-version-zomeversionentry)
- [`DnaEntry`](../Entry_Types.md#dna-dnaentry)
- [`DnaVersionEntry`](../Entry_Types.md#dna-version-dnaversionentry)


### Entry-type Releationship Diagram
![](https://drive.google.com/a/webheroes.ca/thumbnail?id=1VjYZPu0A4-yjhH53Fefr1FEWNdQdgCJ4&sz=w1000)


### `*Response<...>` Wrappers
See [Translation Layer](../API.md#translation-layer) for more info



## Methods

- Agent
  - `whoami( null )`
  - `create_profile( input )`
  - `get_profile( input )`
  - `update_profile( input )`
  - `follow_developer( input )`
  - `unfollow_developer( input )`
  - `get_following( null )`
- Zomes
  - `create_zome( input )`
  - `get_zome( input )`
  - `get_zomes( input )`
  - `get_deprecated_zomes( input )`
  - `get_my_zomes( null )`
  - `get_my_deprecated_zomes( null )`
  - `update_zome( input )`
  - `deprecate_zome( input )`
- Zome Versions
  - `create_zome_version( input )`
  - `get_zome_version( input )`
  - `get_zome_versions( input )`
  - `update_zome_version( input )`
  - `delete_zome_version( input )`
- DNAs
  - `create_dna( input )`
  - `get_dna( input )`
  - `get_dnas( input )`
  - `get_deprecated_dnas( input )`
  - `get_my_dnas( null )`
  - `get_my_deprecated_dnas( null )`
  - `update_dna( input )`
  - `deprecate_dna( input )`
- DNA Versions
  - `create_dna_version( input )`
  - `get_dna_version( input )`
  - `get_dna_versions( input )`
  - `update_dna_version( input )`
  - `delete_dna_version( input )`
- Packaging
  - `get_dna_package( input )`


### Agent

#### `whoami( null ) -> DevHubResponse<AgentInfo>`
Get the agent information for the running cell.

Returns [`AgentInfo`](https://docs.rs/hdk/0.0.*/hdk/prelude/struct.AgentInfo.html) in a
[`DevHubResponse`](../API.md#translation-layer) package

Example response
```javascript
{
    "agent_initial_pubkey": Uint8Array(39) [132, 32, 36, 33, 27, 192, 10, 137, ...],
    "agent_latest_pubkey": Uint8Array(39) [132, 32, 36, 33, 27, 192, 10, 137, ...]
}
```


#### `create_profile( input ) -> EntityResponse<ProfileEntry>`
Create a Profile for the Agent of this cell.

- `input.name` - (*required*) `String`
- `input.avatar_image` - (*required*) `Uint8Array` avatar image byte array
- `input.email` - (*optional*) `String`
- `input.website` - (*optional*) `String`

Returns [`ProfileEntry`](../Entry_Types.md#profile-profileentry) in an
[`EntityResponse`](../API.md#translation-layer) package


#### `get_profile( input ) -> EntityResponse<ProfileEntry>`
Get the latest Profile for a given Agent pubkey.

- `input.agent` - (*optional*) `Uint8Array(39)` an AgentPubKey
  - defaults to the cell Agent

Returns [`ProfileEntry`](../Entry_Types.md#profile-profileentry) in an
[`EntityResponse`](../API.md#translation-layer) package


#### `update_profile( input ) -> EntityResponse<ProfileEntry>`
Update the given properties of a Profile based off of the entry at the given address.

- `input.addr` - (*required*) `Uint8Array(39)` an EntryHash
- `input.properties` - (*required*) `Object` properties to be updated
- `input.properties.name` - (*optional*) `String`
- `input.properties.email` - (*optional*) `String`
- `input.properties.website` - (*optional*) `String`
- `input.properties.avatar_image` - (*optional*) `Uint8Array` avatar image byte array

Returns [`ProfileEntry`](../Entry_Types.md#profile-profileentry) in an
[`EntityResponse`](../API.md#translation-layer) package


#### `follow_developer( input ) -> DevHubResponse<HeaderHash>`
Add the given Agent pubkey to this Agent's following list.

- `input.agent` - (*optional*) `Uint8Array(39)` an AgentPubKey
  - defaults to the cell Agent

Returns a `HeaderHash` in a [`DevHubResponse`](../API.md#translation-layer) package


#### `unfollow_developer( input ) -> DevHubResponse<Option<HeaderHash>>`
Remove the given Agent pubkey from this Agent's following list.

- `input.agent` - (*optional*) `Uint8Array(39)` an AgentPubKey
  - defaults to the cell Agent

If completed, it returns a `HeaderHash` in a [`DevHubResponse`](../API.md#translation-layer)
package.  Otherwise, it returns `null` indicating that the given `AgentPubKey` is already off the
following list.


#### `get_following( null ) -> CollectionResponse<Link>`
Get the list of links to other Agent's root entry.

Returns a list of `Link` values in a [`CollectionResponse`](../API.md#translation-layer) package



### Zomes

#### `create_zome( input ) -> EntityResponse<ZomeEntry>`
Create a new Zome where the Agent of this cell will be used for the `developer`.

- `input.name` - (*required*) `String`
- `input.description` - (*required*) `String`
- `input.display_name` - (*optional*) `String`
- `input.tags` - (*optional*) `Array<String>`
- `input.published_at` - (*optional*) `Number`
  - defaults to the current time
- `input.last_updated` - (*optional*) `Number`
  - defaults to the current time
- `input.metadata` - (*optional*) `Object`

Returns [`ZomeEntry`](../Entry_Types.md#zome-zomeentry) in an
[`EntityResponse`](../API.md#translation-layer) package


#### `get_zome( input ) -> EntityResponse<ZomeEntry>`
Get the latest info for the given Zome ID.

- `input.id` - (*required*) `Uint8Array(39)` an EntryHash

Returns [`ZomeEntry`](../Entry_Types.md#zome-zomeentry) in an
[`EntityResponse`](../API.md#translation-layer) package


#### `get_zomes( input ) -> EntityCollectionResponse<ZomeSummary>`
Get a list of Zomes that were created by the given Agent pubkey.

- `input.agent` - (*optional*) `Uint8Array(39)` an AgentPubKey
  - defaults to the cell Agent

Returns a list of [`ZomeSummary`](../Entry_Types.md#zome-summary-zomesummary) values in an
[`EntityCollectionResponse`](../API.md#translation-layer) package


#### `get_deprecated_zomes( input ) -> EntityCollectionResponse<ZomeSummary>`
Get a list of deprecated Zomes that were created by the given Agent pubkey.

- `input.agent` - (*optional*) `Uint8Array(39)` an AgentPubKey
  - defaults to the cell Agent

Returns a list of [`ZomeSummary`](../Entry_Types.md#zome-summary-zomesummary) values in an
[`EntityCollectionResponse`](../API.md#translation-layer) package


#### `get_my_zomes( null ) -> EntityCollectionResponse<ZomeSummary>`
Alias for `get_zomes( None )`.


#### `get_my_deprecated_zomes( null ) -> EntityCollectionResponse<ZomeSummary>`
Alias for `get_deprecated_zomes( None )`.


#### `update_zome( input ) -> EntityResponse<ZomeEntry>`
Update the given properties of a Zome based off of the entry at the given address.

- `input.addr` - (*required*) `Uint8Array(39)` an EntryHash
- `input.properties` - (*required*) `Object` properties to be updated
- `input.properties.name` - (*optional*) `String`
- `input.properties.display_name` - (*optional*) `String`
- `input.properties.description` - (*optional*) `String`
- `input.properties.tags` - (*optional*) `Array<String>`
- `input.properties.published_at` - (*optional*) `Number`
- `input.properties.last_updated` - (*optional*) `Number`
  - defaults to the current time
- `input.properties.metadata` - (*optional*) `Object`

Returns [`ZomeEntry`](../Entry_Types.md#zome-zomeentry) in an
[`EntityResponse`](../API.md#translation-layer) package


#### `deprecate_zome( input ) -> EntityResponse<ZomeEntry>`
Update the deprecation info of a Zome based off of the entry at the given address.

- `input.addr` - (*required*) `Uint8Array(39)` an EntryHash
- `input.message` - (*required*) `String`

Returns [`ZomeEntry`](../Entry_Types.md#zome-zomeentry) in an
[`EntityResponse`](../API.md#translation-layer) package



### Zome Versions

#### `create_zome_version( input ) -> EntityResponse<ZomeVersionEntry>`
Create a new Zome Version for a specific Zome.

- `input.for_zome` - (*required*) `Uint8Array(39)` an EntryHash
- `input.version` - (*required*) `Number`
- `input.hdk_version` - (*required*) `String`
- (*required*)
  - `input.mere_memory_addr` - (*optional*) `Uint8Array(39)` an EntryHash
  - `input.zome_bytes` - (*optional*) `Uint8Array` WASM bytes
- `input.changelog` - (*optional*) `String`
- `input.published_at` - (*optional*) `Number`
  - defaults to the current time
- `input.last_updated` - (*optional*) `Number`
  - defaults to the current time
- `input.metadata` - (*optional*) `Object`

If a `mere_memory_addr` is not provided, `zome_bytes` must be present.  When `zome_bytes` are
provided, a Mere Memory record is automatically created and the resulting address is stored as the
`mere_memory_addr`.

Returns [`ZomeEntry`](../Entry_Types.md#zome-zomeentry) in an
[`EntityResponse`](../API.md#translation-layer) package


#### `get_zome_version( input ) -> EntityResponse<ZomeVersionEntry>`
Get the latest info for the given Zome Version ID.

- `input.id` - (*required*) `Uint8Array(39)` an EntryHash

Returns [`ZomeVersionEntry`](../Entry_Types.md#zome-version-zomeversionentry) in an
[`EntityResponse`](../API.md#translation-layer) package


#### `get_zome_versions( input ) -> EntityCollectionResponse<ZomeVersionSummary>`
Get a list of Zome Versions that were created for the given Zome ID.

- `input.for_zome` - (*required*) `Uint8Array(39)` an EntryHash

Returns a list of
[`ZomeVersionSummary`](../Entry_Types.md#zome-version-summary-zomeversionsummary) values in an
[`EntityCollectionResponse`](../API.md#translation-layer) package


#### `update_zome_version( input ) -> EntityResponse<ZomeVersionEntry>`
Update the given properties of a Zome based off of the entry at the given address.

- `input.addr` - (*required*) `Uint8Array(39)` an EntryHash
- `input.properties` - (*required*) `Object` properties to be updated
- `input.properties.changelog` - (*optional*) `String`
- `input.properties.published_at` - (*optional*) `Number`
- `input.properties.last_updated` - (*optional*) `Number`
  - defaults to the current time
- `input.properties.metadata` - (*optional*) `Object`

Returns [`ZomeVersionEntry`](../Entry_Types.md#zome-version-zomeversionentry) in an
[`EntityResponse`](../API.md#translation-layer) package


#### `delete_zome_version( input ) -> DevHubResponse<HeaderHash>`
Delete a Zome Version.

- `input.id` - (*required*) `Uint8Array(39)` an EntryHash

Returns a `HeaderHash` in a [`DevHubResponse`](../API.md#translation-layer) package



### DNAs

#### `create_dna( input ) -> EntityResponse<DnaEntry>`
Create a new DNA where the Agent of this cell will be used for the `developer`.

- `input.name` - (*required*) `String`
- `input.description` - (*required*) `String`
- `input.display_name` - (*optional*) `String`
- `input.tags` - (*optional*) `Array<String>`
- `input.icon` - (*optional*) `Uint8Array` the image bytes
- `input.published_at` - (*optional*) `Number`
  - defaults to the current time
- `input.last_updated` - (*optional*) `Number`
  - defaults to the current time
- `input.metadata` - (*optional*) `Object`

Returns [`DnaEntry`](../Entry_Types.md#dna-dnaentry) in an
[`EntityResponse`](../API.md#translation-layer) package


#### `get_dna( input ) -> EntityResponse<DnaEntry>`
Get the latest info for the given DNA ID.

- `input.id` - (*required*) `Uint8Array(39)` an EntryHash

Returns [`DnaEntry`](../Entry_Types.md#dna-dnaentry) in an
[`EntityResponse`](../API.md#translation-layer) package


#### `get_dnas( input ) -> EntityCollectionResponse<DnaSummary>`
Get a list of DNAs that were created by the given Agent pubkey.

- `input.agent` - (*optional*) `Uint8Array(39)` an AgentPubKey
  - defaults to the cell Agent

Returns a list of [`DnaSummary`](../Entry_Types.md#dna-summary-dnasummary) values in an
[`EntityCollectionResponse`](../API.md#translation-layer) package


#### `get_deprecated_dnas( input ) -> EntityCollectionResponse<DnaSummary>`
Get a list of deprecated DNAs that were created by the given Agent pubkey.

- `input.agent` - (*optional*) `Uint8Array(39)` an AgentPubKey
  - defaults to the cell Agent

Returns a list of [`DnaSummary`](../Entry_Types.md#dna-summary-dnasummary) values in an
[`EntityCollectionResponse`](../API.md#translation-layer) package


#### `get_my_dnas( null ) -> EntityCollectionResponse<DnaSummary>`
Alias for `get_dnas( None )`.


#### `get_my_deprecated_dnas( null ) -> EntityCollectionResponse<DnaSummary>`
Alias for `get_deprecated_dnas( None )`.


#### `update_dna( input ) -> EntityResponse<DnaEntry>`
Update the given properties of a DNA based off of the entry at the given address.

- `input.addr` - (*required*) `Uint8Array(39)` an EntryHash
- `input.properties` - (*required*) `Object` properties to be updated
- `input.properties.name` - (*optional*) `String`
- `input.properties.display_name` - (*optional*) `String`
- `input.properties.description` - (*optional*) `String`
- `input.properties.tags` - (*optional*) `Array<String>`
- `input.properties.icon` - (*optional*) `Uint8Array`
- `input.properties.published_at` - (*optional*) `Number`
- `input.properties.last_updated` - (*optional*) `Number`
  - defaults to the current time
- `input.properties.metadata` - (*optional*) `Object`

Returns [`DnaEntry`](../Entry_Types.md#dna-dnaentry) in an
[`EntityResponse`](../API.md#translation-layer) package


#### `deprecate_dna( input ) -> EntityResponse<DnaEntry>`
Update the deprecation info of a DNA based off of the entry at the given address.

- `input.addr` - (*required*) `Uint8Array(39)` an EntryHash
- `input.message` - (*required*) `String`

Returns [`DnaEntry`](../Entry_Types.md#dna-dnaentry) in an
[`EntityResponse`](../API.md#translation-layer) package



### DNA Versions

#### `create_dna_version( input ) -> EntityResponse<DnaVersionEntry>`
Create a new DNA Version for a specific DNA.

- `input.for_dna` - (*required*) `Uint8Array(39)` an EntryHash
- `input.version` - (*required*) `Number`
- `input.hdk_version` - (*required*) `String`
- `input.zomes` - (*required*) `Array`
- `input.zomes[].name` - (*required*) `String`
- `input.zomes[].zome` - (*required*) `Uint8Array(39)` an EntryHash
- `input.zomes[].version` - (*required*) `Uint8Array(39)` an EntryHash
- `input.zomes[].resource` - (*required*) `Uint8Array(39)` an EntryHash
- `input.changelog.` - (*optional*) `String`
- `input.published_at` - (*optional*) `Number`
  - defaults to the current time
- `input.last_updated` - (*optional*) `Number`
  - defaults to the current time
- `input.properties` - (*optional*) `Object`
- `input.metadata` - (*optional*) `Object`

Returns [`DnaVersionEntry`](../Entry_Types.md#dna-version-dnaversionentry) in an
[`EntityResponse`](../API.md#translation-layer) package


#### `get_dna_version( input ) -> EntityResponse<DnaVersionEntry>`
Get the latest info for the given DNA Version ID.

- `input.id` - (*required*) `Uint8Array(39)` an EntryHash

Returns [`DnaVersionEntry`](../Entry_Types.md#dna-version-dnaversionentry) in an
[`EntityResponse`](../API.md#translation-layer) package


#### `get_dna_versions( input ) -> EntityCollectionResponse<DnaVersionSummary>`
Get a list of DNA Versions that were created for the given DNA ID.

- `input.for_dna` - (*required*) `Uint8Array(39)` an EntryHash

Returns a list of
[`DnaVersionSummary`](../Entry_Types.md#dna-version-summary-dnaversionsummary) values in an
[`EntityCollectionResponse`](../API.md#translation-layer) package


#### `update_dna_version( input ) -> EntityResponse<DnaVersionEntry>`
Update the given properties of a DNA based off of the entry at the given address.

- `input.addr` - (*required*) `Uint8Array(39)` an EntryHash
- `input.properties` - (*required*) `Object` properties to be updated
- `input.properties.changelog` - (*optional*) `String`
- `input.properties.published_at` - (*optional*) `Number`
- `input.properties.last_updated` - (*optional*) `Number`
  - defaults to the current time
- `input.properties.metadata` - (*optional*) `Object`

Returns [`DnaVersionEntry`](../Entry_Types.md#dna-version-dnaversionentry) in an
[`EntityResponse`](../API.md#translation-layer) package


#### `delete_dna_version( input ) -> DevHubResponse<HeaderHash>`
Delete a DNA Version.

- `input.id` - (*required*) `Uint8Array(39)` an EntryHash

Returns a `HeaderHash` in a [`DevHubResponse`](../API.md#translation-layer) package



#### Packaging

#### `get_dna_package( input ) -> EntityResponse<DnaVersionPackage>`
Get the latest info for the given DNA Version ID and include the assembled DNA package bytes.

- `input.id` - (*required*) `Uint8Array(39)` an EntryHash

Returns [`DnaVersionPackage`](../Entry_Types.md#dna-version-package-dnaversionpackage) in an
[`EntityResponse`](../API.md#translation-layer) package
