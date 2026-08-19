# owl-structural-model

[![crates.io](https://img.shields.io/crates/v/owl-structural-model.svg)](https://crates.io/crates/owl-structural-model)
[![Documentation](https://docs.rs/owl-structural-model/badge.svg)](https://docs.rs/owl-structural-model)
[![CI](https://github.com/legra-ai/owl-structural-model/actions/workflows/ci.yml/badge.svg)](https://github.com/legra-ai/owl-structural-model/actions/workflows/ci.yml)
[![License](https://img.shields.io/crates/l/owl-structural-model.svg)](https://github.com/legra-ai/owl-structural-model#license)

Typed OWL structural values for Rust.

`owl-structural-model` is a small, dependency-light model for representing
OWL entities, class expressions, literals, annotations, axioms, ontology
identity, and structural patches. It is intended for parsers, editors,
validators, serializers, and storage adapters that need typed OWL values
without taking a dependency on an RDF store or a reasoner.

## What it provides

The crate keeps the structural layer explicit:

- `Entity` and `EntityKind` represent OWL declarations.
- `ClassExpression` and `ObjectPropertyExpression` represent nested OWL
  expressions.
- `OwlLiteral`, `Annotation`, and `AxiomAnnotations` preserve typed values.
- `Axiom` represents the currently supported structural axiom forms.
- `OntologyId` represents anonymous, named, and versioned ontology identity.
- `OntologyPatch` represents a non-empty set of changes to one ontology.

It does not parse files, perform I/O, access a graph store, run a reasoner,
or choose an application-specific profile. A caller can stream a document as
individual axioms and retain only the bounded structural values it needs.

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

## License

Licensed under either of:

- Apache License, Version 2.0
- MIT License

at your option.

