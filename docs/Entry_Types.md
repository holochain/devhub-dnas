[back to API.md](API.md)


# Entry Types
Here we describe the meaning and/or purpose of the fields in our entry types.

- Profile
- Zome
- Zome Version
- DNA
- DNA Version
- hApp
- hApp Release


## Descriptions

### Profile `ProfileEntry`
Source [../devhub_types/src/dnarepo_entry_types.rs](../devhub_types/src/dnarepo_entry_types.rs)

- `name` - The developer's nickname, username, or real name
- `email` - The email that the developer uses for communication
- `website` - A web URL that leads to more information about the developer
- `avatar_image` - A visual representation for the developer


### Zome `ZomeEntry`
Source [../devhub_types/src/dnarepo_entry_types.rs](../devhub_types/src/dnarepo_entry_types.rs)

- `name` - A searchable name intended to match the manifest name
- `display_name` - A non-searchable name for this Zome
- `description` - A short description of this Zome
- `published_at` - A date indicating when this entity was first created
- `last_updated` - A date indicating the last time this entity was updated
- `developer` - Info about the publisher
  - `pubkey` - The `AgentPubKey` of the publisher
- `deprecation` - Deprecation details
  - `message` - A reason for the deprecation
  - `recommended_alternatives` - Entity IDs of other Zomes that may be able to replace this one
- `tags` - A list of strings used for discovery
- `metadata` - Any additional values


### Zome Version `ZomeVersionEntry`
Source [../devhub_types/src/dnarepo_entry_types.rs](../devhub_types/src/dnarepo_entry_types.rs)

- `for_zome` - An Entity ID for the `ZomeEntry` that this version belongs to
- `version` - A value for determining successsion of Zome Versions
- `published_at` - A date indicating when this entity was first created
- `last_updated` - A date indicating the last time this entity was updated
- `changelog` - A spot for describing the changes in this Zome Version
- `mere_memory_addr` - The address of the WASM bytes
- `mere_memory_hash` - The hash of the WASM bytes
- `hdk_version` - The HDK version that built the associated WASM
- `metadata` - Any additional values


### DNA `DnaEntry`
Source [../devhub_types/src/dnarepo_entry_types.rs](../devhub_types/src/dnarepo_entry_types.rs)

- `name` - A searchable name intended to match the manifest name
- `display_name` - A non-searchable name for this DNA
- `description` - A short description of this DNA
- `published_at` - A date indicating when this entity was first created
- `last_updated` - A date indicating the last time this entity was updated
- `developer` - Info about the publisher
  - `pubkey` - The `AgentPubKey` of the publisher
- `icon` - A visual representation for this DNA
- `deprecation` - Deprecation details
  - `message` - A reason for the deprecation
  - `recommended_alternatives` - Entity IDs of other DNAs that may be able to replace this one
- `tags` - A list of strings used for discovery
- `metadata` - Any additional values


### DNA Version `DnaVersionEntry`
Source [../devhub_types/src/dnarepo_entry_types.rs](../devhub_types/src/dnarepo_entry_types.rs)

- `for_dna` - An Entity ID for the `DnaEntry` that this version belongs to
- `version` - A value for determining successsion of DNA Versions
- `published_at` - A date indicating when this entity was first created
- `last_updated` - A date indicating the last time this entity was updated
- `changelog` - A spot for describing the changes in this DNA Version
- `wasm_hash` - A hash of the WASM hashes from each associated Zome Version
- `hdk_version` - The HDK version that built the associated WASM
- `properties` - Values that belong in the DNA manifest properties
- `zomes` - A list containing references to Zome and Zome Version entries
  - `name` - An identifier that is unique within this list of Zomes
  - `zome` - The Entity ID for the referenced Zome
  - `version` - The Entity ID for the specific Zome Version selected
  - `resource` - The address of the WASM bytes for the selected Zome Version
- `metadata` - Any additional values


### hApp `HappEntry`
Source [../devhub_types/src/happ_entry_types.rs](../devhub_types/src/happ_entry_types.rs)

- `title` - A name for this hApp
- `subtitle` - A short description of this hApp
- `description` - A short description of this hApp
- `designer` - The `AgentPubKey` of the publisher
- `published_at` - A date indicating when this entity was first created
- `last_updated` - A date indicating the last time this entity was updated
- `deprecation` - Deprecation details
  - `message` - A reason for the deprecation
  - `recommended_alternatives` - Entity IDs of other hApps that may be able to replace this one
- `icon` - A visual representation for this hApp
- `tags` - A list of strings used for discovery
- `metadata` - Any additional values


### hApp Release `HappReleaseEntry`
Source [../devhub_types/src/happ_entry_types.rs](../devhub_types/src/happ_entry_types.rs)

- `name` - A name for this hApp Release
- `description` - A short description of this hApp Release
- `for_happ` - An Entity ID for the `HappEntry` that this release belongs to
- `published_at` - A date indicating when this entity was first created
- `last_updated` - A date indicating the last time this entity was updated
- `manifest` - The App Manifest details (see
  [`holochain_types/app/AppManifestV1`](https://docs.rs/holochain_types/0.0.*/holochain_types/app/struct.AppManifestV1.html)
  for source material)
  - `manifest_version` - Indicates the manifest format
  - `roles` - A list of Cell configuration
    - `id` - The role ID which will be given to the installed Cell for this DNA
    - `dna` - Info about the DNA for this role
      - `bundled` - An identifier that matches one of `this.dnas[].name`
      - `clone_limit` - The number of clones to be created at runtime
      - `network_seed` - A value for forcing a unique DNA hash
      - `version` - A specific list of compatible DNA Versions
      - `properties` - Implementation specific properties
    - `provisioning` - Provisioning instructions for Holochain
      - `strategy` - One of the supported provisioning strategies
      - `deferred` - ?
  - `name` - The name of the hApp this release belongs to
  - `description` - A description for the hApp this release belongs to
- `dna_hash` - A hash of the WASM hashes from each associated DNA Version
- `hdk_version` - The HDK version that built the associated WASM
- `dnas` - A list containing references to DNA and DNA Version entries
  - `role_name` - An identifier that is unique within this list of DNAs
  - `dna` - The Entity ID for the referenced DNA
  - `version` - The Entity ID for the specific DNA Version selected
  - `wasm_hash` - The `wasm_hash` from the DNA Version entity
- `gui` - Info about the provided GUI
  - `asset_group_id` - The address of the asset bytes
  - `holo_hosting_settings` - Details regarding this hApp's compatibility with Holo Hosting
    - `uses_web_sdk` - A flag indicating the client-side uses the Holo provided Web-SDK
- `metadata` - Any additional values


### File `FileEntry`
Source [../devhub_types/src/web_asset_entry_types.rs](../devhub_types/src/web_asset_entry_types.rs)

- `author` - The `AgentPubKey` of the publisher
- `published_at` - A date indicating when this entity was first created
- `last_updated` - A date indicating the last time this entity was updated
- `file_size` - The total sum of bytes for all the file chunks
- `mere_memory_addr` - The address of a Mere Memory asset
- `mere_memory_hash` - The hash of the Mere Memory content
- `name` - A name for this File
- `metadata` - Any additional values
