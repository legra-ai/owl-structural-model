//! Generators for class and object-property axioms.

use owl_structural_model::{Entity, EntityKind, ObjectPropertyChain, SubObjectPropertyExpression};
use proptest::prelude::*;
use proptest::sample::select;

use super::expression::class_expression;
use super::leaf::{MAX_OPERANDS, iri, object_property};

pub type Axiom = owl_structural_model::Axiom;

const ENTITY_KINDS: [EntityKind; 6] = [
    EntityKind::Class,
    EntityKind::ObjectProperty,
    EntityKind::DataProperty,
    EntityKind::AnnotationProperty,
    EntityKind::NamedIndividual,
    EntityKind::Datatype,
];

fn classes() -> impl Strategy<Value = Vec<owl_structural_model::ClassExpression>> {
    prop::collection::vec(class_expression(), 2..=MAX_OPERANDS)
}

fn properties() -> impl Strategy<Value = Vec<owl_structural_model::ObjectPropertyExpression>> {
    prop::collection::vec(object_property(), 2..=MAX_OPERANDS)
}

fn sub_object_property() -> impl Strategy<Value = SubObjectPropertyExpression> {
    prop_oneof![
        object_property().prop_map(SubObjectPropertyExpression::Property),
        properties().prop_map(|chain| {
            SubObjectPropertyExpression::Chain(ObjectPropertyChain::try_new(chain).unwrap())
        }),
    ]
}

/// One strategy per class axiom family and object-property axiom family.
pub fn schema_axioms() -> Vec<BoxedStrategy<Axiom>> {
    vec![
        (select(ENTITY_KINDS.as_slice()), iri())
            .prop_map(|(kind, iri)| Axiom::declaration(Entity::new(kind, iri)))
            .boxed(),
        (class_expression(), class_expression())
            .prop_map(|(sub, sup)| Axiom::sub_class_of(sub, sup))
            .boxed(),
        classes()
            .prop_map(|c| Axiom::equivalent_classes(c).unwrap())
            .boxed(),
        classes()
            .prop_map(|c| Axiom::disjoint_classes(c).unwrap())
            .boxed(),
        (iri(), classes())
            .prop_map(|(class, c)| Axiom::disjoint_union(class, c).unwrap())
            .boxed(),
        (sub_object_property(), object_property())
            .prop_map(|(sub, sup)| Axiom::sub_object_property_of(sub, sup))
            .boxed(),
        object_property()
            .prop_map(Axiom::functional_object_property)
            .boxed(),
        object_property()
            .prop_map(Axiom::inverse_functional_object_property)
            .boxed(),
        object_property()
            .prop_map(Axiom::reflexive_object_property)
            .boxed(),
        object_property()
            .prop_map(Axiom::irreflexive_object_property)
            .boxed(),
        object_property()
            .prop_map(Axiom::symmetric_object_property)
            .boxed(),
        object_property()
            .prop_map(Axiom::asymmetric_object_property)
            .boxed(),
        object_property()
            .prop_map(Axiom::transitive_object_property)
            .boxed(),
        properties()
            .prop_map(|p| Axiom::equivalent_object_properties(p).unwrap())
            .boxed(),
        properties()
            .prop_map(|p| Axiom::disjoint_object_properties(p).unwrap())
            .boxed(),
        (object_property(), object_property())
            .prop_map(|(first, second)| Axiom::inverse_object_properties(first, second))
            .boxed(),
        (object_property(), class_expression())
            .prop_map(|(property, class)| Axiom::object_property_domain(property, class))
            .boxed(),
        (object_property(), class_expression())
            .prop_map(|(property, class)| Axiom::object_property_range(property, class))
            .boxed(),
    ]
}
