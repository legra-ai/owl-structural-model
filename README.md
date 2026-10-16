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

The target standard is OWL 2, the current version of the W3C Web Ontology
Language. This crate models a deliberately small, currently supported subset
of the OWL 2 structural specification; it is not an OWL 2 parser, reasoner, or
full conformance implementation, and it does not model the older OWL 1 API.

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
- `Axiom` represents the currently supported structural axiom forms.
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
- `OntologyPatch` represents a non-empty set of changes to one ontology.

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
let declaration = Axiom::declaration(customer);
let subclass = Axiom::sub_class_of(
    ClassExpression::named(iri("https://example.org/Customer")),
    ClassExpression::named(iri("https://example.org/Party")),
);

assert!(matches!(declaration, Axiom::Declaration { .. }));
assert!(matches!(subclass, Axiom::SubClassOf { .. }));
```

## Validation

Constructors reject invalid structural shapes immediately. For example,
equivalent classes require at least two operands and language-tagged literals
require a non-empty language tag. Errors are returned as the typed
`StructuralModelError` enum.

## Scope and memory

The crate models individual structural values and explicit, caller-owned
patches. It does not promise that an entire ontology fits in memory. A parser
or application handling an unbounded source should stream axioms and apply
its own explicit limits or persistence strategy.

## Related crates

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
