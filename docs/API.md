[back to README.md](../README.md)


# Overview


## Zomes

- [`dna_library`](./zomes/DNA_Library.md)
- [`happ_library`](./zomes/hApp_Library.md)
- [`web_assets`](./zomes/Web_Assets.md)



## Response layers

- [Translation Layer](#translation-layer) - A layer for communicating the purpose and/or meaning of
  the response
  - Payload Layer - a layer for delivering the content
    - A single item - [`Entity<...>`](https://docs.rs/hc_crud_caps/0.*/hc_crud/struct.Entity.html)
    - Other
      - eg. [AgentInfo](https://docs.rs/hdk/0.0.*/hdk/prelude/struct.AgentInfo.html)


### Translation Layer
A layer for communicating the meaning of the response.

Example `DevHubResponse` success response
```javascript
{
    "type": "success",
    "metadata": {
        "composition": "value"
    },
    "payload": value
}
```

> `DevHubResponse` is an alias for the
[`EssenceResponse<...>`](https://docs.rs/essence_payloads/0.*/essence/enum.EssenceResponse.html)
enum where the success/failure metadata is defined for DevHub.


#### Success response
Success metadata requires a `composition` value which informs the client of the payload's
structure.

Payload compositions values ([source](../devhub_types/src/constants.rs)) and corresponding response
type definitions:

- Value *(an undefined structure)* - `DevHubResponse<...>`
- Value Collection *(a collection of undefined structure)* - `CollectionResponse<...>`
- Entity - `EntityResponse<...>`
- Entity Collection - `EntityCollectionResponse<...>`

Example success response for an `entity` composition
```javascript
{
    "type": "success",
    "metadata": {
        "composition": "entity"
    },
    "payload": {
        "id": EntryHash,
        "header": HeaderHash,
        "address": EntryHash,
        "type": {
            "name": entity_type_name,
            "model": entity_format_name,
        },
        "content": entity_content,
    }
}
```

#### Failure response
No metadata is required



## Related pages

- [Entry Types](./Entry_Types.md)
- [Entity Models](./Entity_Models.md)
