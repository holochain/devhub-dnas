[back to API.md](../API.md)


# `web_assets` - Zome API Reference

## Entry Types Used

- [`FileEntry`](../Entry_Types.md#file-fileentry)


### Entry-type Releationship Diagram
![](https://drive.google.com/a/webheroes.ca/thumbnail?id=1amiyBBUt2JAPz1PhknOv3wFg6v-tKx5I&sz=w1000)


### `*Response<...>` Wrappers
See [Translation Layer](../API.md#translation-layer) for more info



## Methods

- Agent
  - `whoami( null )`
- Files
  - `create_file( input )`
  - `get_file( input )`


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


### Files

#### `create_file( input ) -> EntityResponse<FileEntry>`
Create a new File where the Agent of this cell will be used for the `author`.

- (*required*)
  - `input.mere_memory_addr` - (*optional*) `EntryHash`
  - `input.file_bytes` - (*optional*) `Uint8Array`
- `input.name` - (*optional*) `String`
- `input.published_at` - (*optional*) `Number`
  - defaults to the current time
- `input.last_updated` - (*optional*) `Number`
  - defaults to the current time
- `input.metadata` - (*optional*) `Object`

Returns [`FileEntry`](../Entry_Types.md#file-fileentry) in an
[`EntityResponse`](../API.md#translation-layer) package


#### `get_file( input ) -> EntityResponse<FilePackage>`
Get the latest info for the given File ID.

- `input.id` - (*required*) `Uint8Array(39)` an EntryHash

Returns [`FilePackage`](../Entry_Types.md#file-filepackage) in an
[`EntityResponse`](../API.md#translation-layer) package
