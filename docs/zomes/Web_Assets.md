[back to API.md](../API.md)


# `web_assets` - Zome API Reference

## Entry Types Used

- [`FileEntry`](../Entry_Types.md#file-fileentry)
- [`FileChunkEntry`](../Entry_Types.md#file-chunk-filechunkentry)


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
- File Chunks
  - `create_file_chunk( input )`
  - `get_file_chunk( input )`


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

#### `create_file( input ) -> EntityResponse<FileInfo>`
Create a new File where the Agent of this cell will be used for the `author`.

- `input.file_size` - (*required*) `Number`
- `input.chunk_addresses` - (*required*) `Array` of `Uint8Array(39)` EntryHashes
- `input.name` - (*optional*) `String`
- `input.published_at` - (*optional*) `Number`
  - defaults to the current time

Returns [`FileInfo`](../Entity_Models.md#file-info-fileinfo) in an
[`EntityResponse`](../API.md#translation-layer) package


#### `get_file( input ) -> EntityResponse<FileInfo>`
Get the latest info for the given hApp ID.

- `input.id` - (*required*) `Uint8Array(39)` an EntryHash

Returns [`FileInfo`](../Entity_Models.md#file-info-fileinfo) in an
[`EntityResponse`](../API.md#translation-layer) package


### File Chunks

#### `create_file_chunk( input ) -> EntityResponse<FileChunkEntry>`
Create a File Chunk.

> Is only associated with a File via the `chunk_addresses` reference in the FileEntry

- `input.sequence` - (*required*) `Object`
- `input.sequence.position` - (*required*) `Number`
- `input.sequence.length` - (*required*) `Number`
- `input.bytes` - (*required*) `Uint8Array`

Returns [`FileChunkEntry`](../Entry_Types.md#file-chunk-filechunkentry) in an
[`EntityResponse`](../API.md#translation-layer) package


#### `get_file_chunk( input ) -> EntityResponse<FileChunkEntry>`
Get the entry for the given File Chunk address.

- `input.addr` - (*required*) `Uint8Array(39)` an EntryHash

Returns [`FileChunkEntry`](../Entry_Types.md#file-chunk-filechunkentry) in an
[`EntityResponse`](../API.md#translation-layer) package
