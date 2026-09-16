# fixtures/

`<contract>/` directories, each with at least one `valid-*.json` and one
`invalid-*.json`. Every fixture round-trips: deserialize → schema-validate →
serialize == input (after canonicalization). The first fixtures are the
N-PORT identity-closure inquiry and its hand-issued grant from
`SaltIO/evidence-machine` `docs/MILESTONE_1.md`.
