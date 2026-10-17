# OWL 2 structural coverage

The reference is the [OWL 2 Structural Specification, second edition](https://www.w3.org/TR/owl2-syntax/), particularly sections 2.1 and 13. The counts below describe representation and local construction/deserialization invariants. They do not certify an OWL parser, profile checker or reasoner.

| Structural area | Representation | Executable evidence |
|---|---|---|
| Entity categories | All six declaration categories | `serde_round_trips.rs` |
| Class expressions | All 18 grammar forms, including all object/data cardinalities | `complete_expressions.rs` |
| Individuals | Named and scoped anonymous individuals; caller-selected anonymous identity | `complete_grammar.rs`, `shared_rdf.rs` |
| Object properties | Named and inverse-of-named expressions; ordered subproperty chains | `property_axioms.rs`, `complete_expressions.rs` |
| Data ranges | All six standard unary forms and facet/value pairs | `data_ranges.rs` |
| Axiom families | All 37 standard families, including all seven assertion kinds and all four annotation-axiom kinds | `complete_grammar.rs` |
| Annotations | Ontology/axiom annotation values and recursively annotated annotations | `complete_grammar.rs`, `structural_identity.rs` |
| Ontology identity and changes | Anonymous/named/versioned IDs; imports, annotations and axioms in ordered patches | `serde_round_trips.rs` |
| Local arity | Protected minimum occurrence counts for Boolean expressions, enumerations, n-ary axioms, facets and ordered property lists | `complete_arities.rs`, `serde_invariants.rs` |
| Structural identity | Unordered association equality/hash; ordered chains and assertion positions; exact literal identity | `structural_identity.rs`, `property_axioms.rs` |
| Generic and borrowed values | Literals/anonymous identities propagate through class expressions, ranges, annotations, assertions and patches | `shared_rdf.rs` |
| Serialization | JSON and non-self-describing Postcard; checked nested input and rejected unknown semantic fields | All integration suites, especially `serde_strictness.rs` |

The class-expression inventory is `Class`, four Boolean/enumeration constructors, two object quantifiers, object value/self restrictions, three object cardinalities, two data quantifiers, a data value restriction and three data cardinalities. Built-in classes are named IRIs, not additional expression categories. `ClassExpressionKind`, `AxiomKind` and `DataRangeKind` expose the complete catalogs for consumers.

Arity is checked against source occurrences in the functional grammar. Repeated operands can occur in source syntax. Equality and hashing treat unordered associations as sets, while iteration and serialization retain source occurrences. These bytes are not canonical identities. Cardinalities are normalized integer values; literal lexical forms are not normalized. Omitting a cardinality qualifier constructs its explicit standard filler.

Each model value and patch is caller-budgeted. The crate does not collect an unbounded ontology or import closure. Consumers should stream axioms into their own storage and bound input size and nesting depth before materializing individual values.

## Responsibilities outside this model

- Parsing or rendering Functional-Style Syntax, OWL/XML, Manchester Syntax, RDF/XML, Turtle, TriG, N-Triples or N-Quads.
- Mapping RDF graphs to/from OWL structures, including blank-node matching and annotation reification. Representing RDF 1.2 directional literals does not make them OWL 2 DL literals or assign OWL semantics to triple terms.
- Evaluating datatype lexical/value/facet spaces, including custom datatype maps and n-ary datatype arity.
- Ontology-wide OWL 2 DL restrictions: declaration consistency, legal punning, datatype-definition dependencies, simple-property usage, property-chain regularity, anonymous-individual placement and other axiom-closure requirements.
- Complete EL/QL/RL membership checking, consistency, satisfiability, classification, realization, entailment and RDF closure execution.
- W3C end-to-end conformance certification and comparison against independent OWL implementations.

Consumers must implement those capabilities or report their absence explicitly. Successful model construction or Serde round trips are not evidence that those operations have run. SWRL and OWLAPI-specific application services are outside the OWL 2 structural grammar.
