//! Bounded-depth generators for class expressions and data ranges.

use owl_structural_model::FacetRestriction;
use proptest::prelude::*;
use proptest::strategy::Union;

use super::leaf::{
    MAX_OPERANDS, cardinality, datatype_iri, facet_iri, individual, iri, literal, object_property,
};

pub type ClassExpression = owl_structural_model::ClassExpression;
pub type DataRange = owl_structural_model::DataRange;

/// The deepest nesting generated for any recursive expression.
const MAX_DEPTH: u32 = 3;
/// The expected operands per recursive node, matching `MAX_OPERANDS`.
const BRANCHING: u32 = 4;

fn operands<T: std::fmt::Debug>(
    element: impl Strategy<Value = T>,
    minimum: usize,
) -> impl Strategy<Value = Vec<T>> {
    prop::collection::vec(element, minimum..=MAX_OPERANDS)
}

fn data_range_leaf() -> impl Strategy<Value = DataRange> {
    prop_oneof![
        datatype_iri().prop_map(DataRange::datatype),
        operands(literal(), 1).prop_map(|literals| DataRange::data_one_of(literals).unwrap()),
        (
            datatype_iri(),
            operands(
                (facet_iri(), literal())
                    .prop_map(|(facet, value)| FacetRestriction::new(facet, value)),
                1
            ),
        )
            .prop_map(
                |(datatype, facets)| DataRange::datatype_restriction(datatype, facets).unwrap()
            ),
    ]
}

/// Data ranges nested at most three levels deep.
pub fn data_range() -> impl Strategy<Value = DataRange> {
    data_range_leaf().prop_recursive(MAX_DEPTH, 24, BRANCHING, |inner| {
        prop_oneof![
            operands(inner.clone(), 2)
                .prop_map(|ranges| DataRange::data_intersection_of(ranges).unwrap()),
            operands(inner.clone(), 2).prop_map(|ranges| DataRange::data_union_of(ranges).unwrap()),
            inner.prop_map(DataRange::data_complement_of),
        ]
    })
}

fn data_class_leaf() -> Union<BoxedStrategy<ClassExpression>> {
    let range = || prop::option::of(data_range());
    Union::new([
        (operands(iri(), 1), data_range())
            .prop_map(|(properties, range)| {
                ClassExpression::data_some_values_from(properties, range).unwrap()
            })
            .boxed(),
        (operands(iri(), 1), data_range())
            .prop_map(|(properties, range)| {
                ClassExpression::data_all_values_from(properties, range).unwrap()
            })
            .boxed(),
        (iri(), literal())
            .prop_map(|(property, value)| ClassExpression::data_has_value(property, value))
            .boxed(),
        (cardinality(), iri(), range())
            .prop_map(|(count, property, range)| {
                ClassExpression::data_min_cardinality(count, property, range)
            })
            .boxed(),
        (cardinality(), iri(), range())
            .prop_map(|(count, property, range)| {
                ClassExpression::data_max_cardinality(count, property, range)
            })
            .boxed(),
        (cardinality(), iri(), range())
            .prop_map(|(count, property, range)| {
                ClassExpression::data_exact_cardinality(count, property, range)
            })
            .boxed(),
    ])
}

fn class_leaf() -> impl Strategy<Value = ClassExpression> {
    prop_oneof![
        iri().prop_map(ClassExpression::named),
        Just(ClassExpression::thing()),
        operands(individual(), 1)
            .prop_map(|members| ClassExpression::object_one_of(members).unwrap()),
        (object_property(), individual())
            .prop_map(|(property, member)| ClassExpression::object_has_value(property, member)),
        object_property().prop_map(ClassExpression::object_has_self),
        data_class_leaf(),
    ]
}

fn object_class_branch(
    inner: &BoxedStrategy<ClassExpression>,
) -> Union<BoxedStrategy<ClassExpression>> {
    let filler = || prop::option::of(inner.clone());
    Union::new([
        operands(inner.clone(), 2)
            .prop_map(|classes| ClassExpression::object_intersection_of(classes).unwrap())
            .boxed(),
        operands(inner.clone(), 2)
            .prop_map(|classes| ClassExpression::object_union_of(classes).unwrap())
            .boxed(),
        inner
            .clone()
            .prop_map(ClassExpression::object_complement_of)
            .boxed(),
        (object_property(), inner.clone())
            .prop_map(|(property, filler)| {
                ClassExpression::object_some_values_from(property, filler)
            })
            .boxed(),
        (object_property(), inner.clone())
            .prop_map(|(property, filler)| {
                ClassExpression::object_all_values_from(property, filler)
            })
            .boxed(),
        (cardinality(), object_property(), filler())
            .prop_map(|(count, property, filler)| {
                ClassExpression::object_min_cardinality(count, property, filler)
            })
            .boxed(),
        (cardinality(), object_property(), filler())
            .prop_map(|(count, property, filler)| {
                ClassExpression::object_max_cardinality(count, property, filler)
            })
            .boxed(),
        (cardinality(), object_property(), filler())
            .prop_map(|(count, property, filler)| {
                ClassExpression::object_exact_cardinality(count, property, filler)
            })
            .boxed(),
    ])
}

/// Class expressions nested at most three levels deep.
pub fn class_expression() -> impl Strategy<Value = ClassExpression> {
    class_leaf().prop_recursive(MAX_DEPTH, 24, BRANCHING, |inner| {
        object_class_branch(&inner.boxed())
    })
}
