# evidence-protocol

Public wire contracts for the CMD+RVL **Evidence Machine**: the `*.v0` types,
their JSON Schemas, and fixtures. Defined once, here, so that the private
runtime can implement them without redefining them and the rules layer can
check conformance against them.

Status: **v0.5.0 prepared for coordinator review.** `inquiry.v0`,
`grant.v0`, `artifact.v0`, `plan.v0`, `plan_patch.v0`, `capability.v0`,
`receipt.v0`, `run.v0`, and `answer.v0` now carry the Milestone 1
consumer-backed fields, schemas, and fixtures. Remaining contract fields land
only as their consumers land.

## The three-layer split

```text
evidence-protocol   public   the wire types, schemas, fixtures      (this repo)
spine-rules         public   conformance rules + operator.v0        (consumes protocol)
evidence-machine    private  the runtime: compile, run, seal        (consumes both)

Foundry -> evidence-protocol -> Evidence Machine API
evidence-machine -> evidence-protocol
spine-rules -> evidence-protocol
public operators -> spine-rules -> evidence-protocol
public operators -X-> evidence-machine
spine-rules      -X-> evidence-machine
evidence-protocol -X-> evidence-machine, spine-rules, any operator
```

The last three lines are enforced, not assumed. `deny.toml` bans the private
runtime and the rules layer from this crate's dependency graph and forbids all
git sources; CI runs `cargo deny check bans sources licenses` on every push.
The private runtime carries the mirror check: every `schema` string it emits
must resolve to a type re-exported from this crate.

## Initial surface

| Contract | Answers |
|---|---|
| `inquiry.v0` | what is being asked, over what universe, under what requested authority |
| `grant.v0` | what this run is actually allowed to do (opaque capability handles, never credentials) |
| `artifact.v0` | what moves between nodes: the content-addressed envelope |
| `plan.v0` / `plan_patch.v0` | how the inquiry will be established: a compiled, content-addressed DAG |
| `receipt.v0` / `run.v0` | what actually happened, on four independent axes |
| `answer.v0` | what is justified, with a coverage certificate and typed unknowns |
| `capability.v0` | what one registered evidence channel can prove, and how independent it is |

`operator.v0` is **not** here; it belongs to
[`spine-rules`](https://github.com/cmdrvl/spine-rules).

Deferred until a consumer exists: `control.v0`, identity edge and mesh,
strategy and tournament, promotion, ontology contracts. See `CONTRIBUTING.md`
for the rule.

## Where the design lives

The target architecture is *The Evidence Machine* in the CMD+RVL living spec
(`cmdrvl-context/docs/02-architecture/evidence-machine.md`, §4 for the
contracts, §2 for the boundary rules). The first vertical slice that drives
which fields land first is Milestone 1 in `SaltIO/evidence-machine`:
historical instrument identity closure over N-PORT-P holdings.

## Using it

```toml
[dependencies]
evidence-protocol = { git = "https://github.com/cmdrvl/evidence-protocol", tag = "v0.5.0" }
```

```rust
use evidence_protocol::{v0, CONTRACTS};
assert!(CONTRACTS.contains(&v0::inquiry::SCHEMA));
```

## License

MIT.
