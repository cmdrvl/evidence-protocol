# fixtures/

Contract fixture directories, each with at least one valid JSON file and one
`invalid-*.json`. The Milestone 1 valid files keep the consuming runtime's
documented paths:

- `fixtures/inquiries/nport_identity_closure.v0.json`
- `fixtures/grants/nport_identity_closure.v0.json`

Every valid fixture round-trips: deserialize → schema-validate →
canonicalize → serialize equals the canonicalized input. Apart from the file's
terminal newline, valid fixtures are stored as canonical JSON bytes: sorted
keys and no insignificant whitespace.
