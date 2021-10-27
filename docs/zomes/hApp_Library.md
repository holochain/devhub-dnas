[back to API.md](../API.md)


# `happ_library` - Zome API Reference

## Entry Types Used

- [`HappEntry`](../Entry_Types.md#happ-happentry)
- [`HappReleaseEntry`](../Entry_Types.md#happ-release-happreleaseentry)


### Entry-type Releationship Diagram
![](https://drive.google.com/a/webheroes.ca/thumbnail?id=1amiyBBUt2JAPz1PhknOv3wFg6v-tKx5I&sz=w1000)


### `*Response<...>` Wrappers
See [Translation Layer](../API.md#translation-layer) for more info



## Methods

- Agent
  - `whoami( null )`
- hApps
  - `create_happ( input )`
  - `get_happ( input )`
  - `update_happ( input )`
  - `deprecate_happ( input )`
  - `get_happs( input )`
  - `get_my_happs(  null )`
- hApp Releases
  - `create_happ_release(input )`
  - `get_happ_release( input )`
  - `update_happ_release( input )`
  - `delete_happ_release( input )`
  - `get_happ_releases( input )`
- Packaging
  - `get_gui( input )`
  - `get_release_package( input )`


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


### hApps

#### `create_happ( input ) -> EntityResponse<HappInfo>`
Create a new hApp where the Agent of this cell will be used for the `designer.pubkey`.

- `input.title` - (*required*) `String`
- `input.subtitle` - (*required*) `String`
- `input.description` - (*required*) `String`
- `input.thumbnail_image` - (*optional*) `Uint8Array` the image bytes
- `input.published_at` - (*optional*) `Number`
  - defaults to the current time
- `input.last_updated` - (*optional*) `Number`
  - defaults to the current time
- `input.gui` - (*optional*) `Object`
- `input.gui.asset_group_id` - (*required*) `Uint8Array(39)` an EntryHash
- `input.gui.uses_web_sdk` - (*required*) `Boolean`

Returns [`HappInfo`](../Entity_Models.md#happ-info-happinfo) in an
[`EntityResponse`](../API.md#translation-layer) package


#### `get_happ( input ) -> EntityResponse<HappInfo>`
Get the latest info for the given hApp ID.

- `input.id` - (*required*) `Uint8Array(39)` an EntryHash

Returns [`HappInfo`](../Entity_Models.md#happ-info-happinfo) in an
[`EntityResponse`](../API.md#translation-layer) package


#### `get_happs( input ) -> EntityCollectionResponse<HappSummary>`
Get a list of hApps that were created by the given Agent pubkey.

- `input.agent` - (*optional*) `Uint8Array(39)` an AgentPubKey
  - defaults to the cell Agent

Returns a list of [`HappSummary`](../Entity_Models.md#happ-summary-happsummary) values in an
[`EntityCollectionResponse`](../API.md#translation-layer) package


#### `get_my_happs( null ) -> EntityCollectionResponse<HappSummary>`
Alias for `get_happs( None )`.


#### `update_happ( input ) -> EntityResponse<HappInfo>`
Update the given properties of a hApp based off of the entry at the given address.

- `input.addr` - (*required*) `Uint8Array(39)` an EntryHash
- `input.properties` - (*required*) `Object` properties to be updated
- `input.properties.title` - (*optional*) `String`
- `input.properties.subtitle` - (*optional*) `String`
- `input.properties.description` - (*optional*) `String`
- `input.properties.thumbnail_image` - (*optional*) `Uint8Array` the image bytes
- `input.properties.published_at` - (*optional*) `Number`
- `input.properties.last_updated` - (*optional*) `Number`
  - defaults to the current time
- `input.properties.gui` - (*optional*) `Object`
- `input.properties.gui.asset_group_id` - (*required*) `Uint8Array(39)` an EntryHash
- `input.properties.gui.uses_web_sdk` - (*required*) `Boolean`

Returns [`HappInfo`](../Entity_Models.md#happ-info-happinfo) in an
[`EntityResponse`](../API.md#translation-layer) package


#### `deprecate_happ( input ) -> EntityResponse<HappInfo>`
Update the deprecation info of a hApp based off of the entry at the given address.

- `input.addr` - (*required*) `Uint8Array(39)` an EntryHash
- `input.message` - (*required*) `String`

Returns [`HappInfo`](../Entity_Models.md#happ-info-happinfo) in an
[`EntityResponse`](../API.md#translation-layer) package



### hApp Releases

#### `create_happ_release( input ) -> EntityResponse<HappReleaseInfo>`
Create a new hApp Release for a specific hApp.

- `input.for_happ` - (*required*) `Uint8Array(39)` an EntryHash
- `input.name` - (*required*) `String`
- `input.description` - (*required*) `String`
- `input.manifest` - (*required*) `Object`
- `input.manifest.manifest_version` - (*required*) `String`
- `input.manifest.slots` - (*required*) `Object`
- `input.manifest.slots[].id` - (*required*) `String`
- `input.manifest.slots[].dna` - (*required*) `Object`
- `input.manifest.slots[].dna.bundled` - (*required*) `String`
- `input.manifest.slots[].dna.clone_limit` - (*required*) `Number`
- `input.manifest.slots[].dna.uid` - (*optional*) `String`
- `input.manifest.slots[].dna.version` - (*optional*) `String`
- `input.manifest.slots[].dna.properties` - (*optional*) `Object` any key/values allowed
- `input.manifest.slots[].provisioning` - (*optional*) `Object`
- `input.manifest.slots[].provisioning.strategy` - (*required*) `String`
- `input.manifest.slots[].provisioning.deferred` - (*required*) `Boolean`
- `input.manifest.name` - (*optional*) `String`
- `input.manifest.description` - (*optional*) `String`
- `input.dnas` - (*required*) `Array`
- `input.dnas[].name` - (*required*) `String`
- `input.dnas[].dna` - (*required*) `Uint8Array(39)` an EntryHash
- `input.dnas[].version` - (*required*) `Uint8Array(39)` an EntryHash
- `input.published_at` - (*optional*) `Number`
  - defaults to the current time
- `input.last_updated` - (*optional*) `Number`
  - defaults to the current time

Returns [`HappReleaseInfo`](../Entity_Models.md#happ-release-info-happreleaseinfo) in an
[`EntityResponse`](../API.md#translation-layer) package


#### `get_happ_release( input ) -> EntityResponse<HappReleaseInfo>`
Get the latest info for the given hApp Release ID.

- `input.id` - (*required*) `Uint8Array(39)` an EntryHash

Returns [`HappReleaseInfo`](../Entity_Models.md#happ-release-info-happreleaseinfo) in an
[`EntityResponse`](../API.md#translation-layer) package


#### `get_happ_releases( input ) -> EntityCollectionResponse<HappReleaseSummary>`
Get a list of hApp Releases that were created for the given hApp ID.

- `input.for_happ` - (*required*) `Uint8Array(39)` an EntryHash

Returns a list of
[`HappReleaseSummary`](../Entity_Models.md#happ-release-summary-happreleasesummary) values in an
[`EntityCollectionResponse`](../API.md#translation-layer) package


#### `update_happ_release( input ) -> EntityResponse<HappReleaseInfo>`
Update the given properties of a hApp based off of the entry at the given address.

- `input.addr` - (*required*) `Uint8Array(39)` an EntryHash
- `input.properties` - (*required*) `Object` properties to be updated
- `input.properties.name` - (*optional*) `String`
- `input.properties.description` - (*optional*) `String`
- `input.properties.published_at` - (*optional*) `Number`
- `input.properties.last_updated` - (*optional*) `Number`
  - defaults to the current time

Returns [`HappReleaseInfo`](../Entity_Models.md#happ-release-info-happreleaseinfo) in an
[`EntityResponse`](../API.md#translation-layer) package


#### `delete_happ_release( input ) -> DevHubResponse<HeaderHash>`
Delete a hApp Release.

- `input.id` - (*required*) `Uint8Array(39)` an EntryHash

Returns a `HeaderHash` in a [`DevHubResponse`](../API.md#translation-layer) package



### Packaging

#### `get_release_package( input ) -> DevHubResponse<Vec<u8>>`
Get the assembled hApp package bytes for the given hApp Release ID.

- `input.id` - (*required*) `Uint8Array(39)` an EntryHash
- `input.dna_hash` - (*required*) `Uint8Array(39)` an DnaHash

Returns `Uint8Array` in a [`DevHubResponse`](../API.md#translation-layer) package
