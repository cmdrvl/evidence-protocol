# schemas/

One JSON Schema per contract, named `<contract>.v0.schema.json`, added in the
same change as the serde type it describes and the fixtures that exercise it.
Schemas are public protocol artifacts and reject fields outside their current
consumer-backed v0 surface.
