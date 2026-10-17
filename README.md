# owl-structural-model

[![crates.io](https://img.shields.io/crates/v/owl-structural-model.svg)](https://crates.io/crates/owl-structural-model)
[![Documentation](https://docs.rs/owl-structural-model/badge.svg)](https://docs.rs/owl-structural-model)
[![CI](https://github.com/legra-ai/owl-structural-model/actions/workflows/ci.yml/badge.svg)](https://github.com/legra-ai/owl-structural-model/actions/workflows/ci.yml)
[![License](https://img.shields.io/crates/l/owl-structural-model.svg)](https://github.com/legra-ai/owl-structural-model#license)

Typed OWL 2 structural values for Rust.

`owl-structural-model` is a small, dependency-light model for representing
the structural values of OWL 2: entities, class expressions, literals,
annotations, axioms, ontology identity, and structural patches. It is intended
for parsers, editors, validators, serializers, and storage adapters that need
typed OWL values without taking a dependency on an RDF store or a reasoner.

The model represents all 37 OWL 2 axiom families, all 18 class-expression forms,
all six unary data-range forms, individuals and recursively nested annotations.
Its target is the W3C OWL 2 structural grammar. Structural representation is
separate from concrete-syntax parsing, RDF mapping, datatype evaluation,
ontology-wide OWL 2 DL validation and reasoning. Those execution layers are not
implemented by this crate. See [coverage and boundaries](COMPLETENESS.md).

## W3C specifications

- [OWL 2 Web Ontology Language Document Overview](https://www.w3.org/TR/owl-overview/)
- [OWL 2 Structural Specification and Functional-Style Syntax](https://www.w3.org/TR/owl2-syntax/)
- [OWL 2 Mapping to RDF Graphs](https://www.w3.org/TR/owl2-mapping-to-rdf/)

## What it provides

The crate keeps the structural layer explicit:

- `Entity` and `EntityKind` represent OWL declarations.
- `ClassExpression` and `ObjectPropertyExpression` represent nested OWL
  expressions.
- `OwlLiteral`, `Annotation`, and `AxiomAnnotations` preserve typed values.
- `DataRange` represents named datatypes, intersections, unions, complements,
  literal enumerations and facet restrictions. Constructors and deserialization
  check structural arity; datatype value evaluation stays outside the model.
- `Axiom` represents every standard axiom family, including individual assertions,
  negative assertions, disjoint unions and annotation-property axioms.
- `Individual<B>` distinguishes named and caller-owned anonymous individuals.
- `ClassExpression<L, B>` covers Boolean expressions, individual enumerations,
  object/data restrictions and all cardinality restrictions.
- `Cardinality` preserves arbitrary-size nonnegative integer values without a
  machine-word limit. Omitted qualifiers become explicit `owl:Thing` or
  `rdfs:Literal` fillers; RDF literal lexical forms remain untouched.
- `AxiomKind`, `ClassExpressionKind` and `DataRangeKind` provide reusable typed
  visitor/diagnostic categories.
- Property axioms include all seven object-property characteristics, functional
  data properties, and object subproperty inclusions with either a single
  property expression or an ordered `ObjectPropertyChain`. Chains preserve
  repeated members and require at least two members, including on deserialization.
- Object and data property axioms include equivalence, disjointness, domains,
  ranges, and subproperty inclusions; object properties also support inverse
  relationships. Datatype definitions retain their structural `DataRange`.
- `KeyProperties` keeps object and data key properties separate and rejects a
  key with neither, including during deserialization. `Axiom::has_key` associates
  those properties with a class expression; it does not execute key reasoning.
- `OntologyId` represents anonymous, named, and versioned ontology identity.
- `OntologyPatch` represents a non-empty ordered sequence of changes to one ontology.

It does not parse files, perform I/O, access a graph store, run a reasoner,
or choose an application-specific profile. A caller can stream a document as
individual axioms and retain only the bounded structural values it needs.

The structural layer is shared vocabulary for editors and reasoner adapters.
Its constructors enforce local shape invariants; ontology-wide OWL 2 DL
restrictions, profile membership, RDF mapping, and RDF closure execution are
separate responsibilities. An RDF reasoner need not convert every input triple
to a structural axiom before applying OWL-RL/RDF rules.

Property chains follow [OWL 2 object subproperty axioms](https://www.w3.org/TR/owl2-syntax/#Object_Subproperties).
All axioms expose `annotations()` and `with_annotations()` to inspect or replace
axiom annotations without changing the structural operands.

## Example

```rust
use iri_rfc3987::Iri;
use owl_structural_model::{Axiom, ClassExpression, Entity};

fn iri(value: &str) -> Iri<String> {
    Iri::parse(value).expect("example IRI is valid").into()
}

let customer = Entity::class(iri("https://example.org/Customer"));
let declaration: Axiom = Axiom::declaration(customer);
let subclass: Axiom = Axiom::sub_class_of(
    ClassExpression::named(iri("https://example.org/Customer")),
    ClassExpression::named(iri("https://example.org/Party")),
);

assert!(matches!(declaration, Axiom::Declaration { .. }));
assert!(matches!(subclass, Axiom::SubClassOf { .. }));
```

## Validation

Constructors reject invalid structural shapes immediately. For example,
equivalent classes require at least two operands. Structural errors use
`StructuralModelError`; shared RDF constructors return `rdf_structural_model::ModelError`.

## Serialization

All semantic types implement Serde serialization and deserialization directly,
without a JSON-specific representation. The integration suites exercise JSON
and Postcard, including every supported axiom and expression variant, ontology
identities, annotations and all patch operations. Nested literals retain their
exact lexical form and language metadata.

Deserialization enforces the same local checks as constructors: operand counts,
well-formed language tags, coherent literal datatype/facets, scoped non-empty
anonymous identities, valid ontology identity combinations, and non-empty patches.
This does not evaluate datatype lexical spaces or validate OWL profiles.
Arity-bearing enum fields use checked `OperandSet`/`OperandList` types, so direct
enum assembly cannot bypass their minimum occurrence counts. Unknown fields
in semantic objects and unknown enum variants fail during deserialization.
Caller-selected payload types enforce their own invariants.

Serde encodings do not implement RDF concrete syntaxes or OWL Functional-Style
Syntax, and do not define canonical identity bytes or a stable storage protocol.
Postcard is a test dependency, not a runtime encoding requirement.

## Shared RDF values and caller representations

`OwlLiteral` is a re-export of `rdf_structural_model::Literal`, and `BlankNodeId`
is the shared scoped anonymous identity. Literal lexical forms remain exact;
language-tag identity ignores ASCII case while serialization preserves spelling.
Anonymous identities include an application-assigned scope IRI; identical local
labels from different scopes are distinct. No global identifier allocation occurs.

`Annotation<L, B>`, `AnnotationValue<L, B>`, `AxiomAnnotations<L, B>`,
`Axiom<L, B>`, `OntologyPatch<L, B>` and `OntologyPatchOperation<L, B>` accept
caller-owned literal and anonymous representations. `ClassExpression<L, B>`, `Individual<B>`, `AnnotationSubject<B>` and
`DataRange<L>`/`FacetRestriction<L>` propagate the corresponding representation.
Defaults are the shared RDF literal and scoped blank node. Named OWL entities
and property IRIs remain validated `Iri<String>` values. Construction imposes
no serialization or cloning requirements on caller-supplied payloads; Serde and
other derived traits are conditional on their corresponding traits.

The shared literal default also represents RDF 1.2 directional language strings.
Representability does not imply OWL 2 profile membership or inference support.
Callers supplying custom values own their validation and semantic contracts;
serialized payloads retain their selected type without JSON intermediates.

Use `LanguageTag::try_new` before `OwlLiteral::language_tagged`, handle the result
of `OwlLiteral::typed`, and supply `BlankNodeId::try_new(scope, label)` for anonymous
individuals. Give generic constructors a result type when inputs do not determine
the literal and anonymous types, such as `let axiom: Axiom = Axiom::declaration(entity)`.

## Structural identity

Unordered OWL associations use `OperandSet`; equality and hashing ignore source
order and repeated equal members, including nested annotations, keys and data
ranges. `UnorderedPair` gives inverse-property axioms unordered identity with
exactly two source occurrences. Property chains, patch operations and the property
positions of n-ary data restrictions retain order and repetition.

Source occurrence order remains available for iteration and Serde. `MIN` checks
functional-syntax occurrence arity; source syntax may repeat a member. Serde is
not a canonical encoding. Equality relies on the caller's payload equality,
which must represent its intended structural identity. Hashing relies on the
corresponding `Hash` contract. This is structural equality, not entailment or
datatype value equality.

`owl:Thing` and `owl:Nothing` each have one representation as named class IRIs.
Use `ClassExpression::thing()`/`nothing()` and `is_thing()`/`is_nothing()` for their
standard construction and classification. Inverses contain a named property IRI,
rather than recursively nesting inverse expressions.

## Scope and memory

The crate models individual structural values and explicit, caller-owned
patches. It does not promise that an entire ontology fits in memory. A parser
or application handling an unbounded source should stream axioms and apply
its own explicit limits or persistence strategy.

## Related crates

- [`rdf-structural-model`](https://crates.io/crates/rdf-structural-model) —
  shared RDF literals, scoped blank nodes, triples/quads and borrowed values.
- [`iri-rfc3987`](https://crates.io/crates/iri-rfc3987) — typed IRI parsing,
  validation, and resolution.
- [`iri-namespace`](https://crates.io/crates/iri-namespace) — validated RDF
  namespaces, prefixes, and CURIEs.
- [`rdf-winnow`](https://crates.io/crates/rdf-winnow) — reusable RDF lexical
  parsers built on `winnow`.
- [`shacl-structural-model`](https://crates.io/crates/shacl-structural-model)
  — typed structural values for SHACL shapes.
- [`sparql-results`](https://crates.io/crates/sparql-results) — async streaming
  SPARQL Results XML and JSON serialization.

## License

Licensed under either of:

- Apache License, Version 2.0
- MIT License

at your option.
