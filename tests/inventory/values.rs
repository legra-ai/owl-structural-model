//! Shared operands for the per-family axiom fixtures.

use iri_rfc3987::Iri;
use owl_structural_model::{Individual, ObjectPropertyExpression, OwlDatatype};
use rdf_structural_model::{BlankNodeId, Literal};
pub type Axiom = owl_structural_model::Axiom;
pub type Annotation = owl_structural_model::Annotation;
pub type AnnotationSubject = owl_structural_model::AnnotationSubject;
pub type AnnotationValue = owl_structural_model::AnnotationValue;
pub type AxiomAnnotations = owl_structural_model::AxiomAnnotations;
pub type ClassExpression = owl_structural_model::ClassExpression;
pub type DataRange = owl_structural_model::DataRange;
pub fn iri(value: &str) -> Iri<String> {
    Iri::parse(value).unwrap().into()
}
pub fn class() -> ClassExpression {
    ClassExpression::named(iri("urn:C"))
}
pub fn property() -> ObjectPropertyExpression {
    ObjectPropertyExpression::named(iri("urn:p"))
}
pub fn range() -> DataRange {
    DataRange::datatype(OwlDatatype::String.into())
}
pub fn literal() -> Literal {
    Literal::string("exact value")
}
pub fn blank() -> BlankNodeId {
    BlankNodeId::try_new(iri("urn:scope"), "b0".to_owned()).unwrap()
}
pub fn individual() -> Individual {
    Individual::named(iri("urn:individual"))
}
pub fn anonymous() -> Individual {
    Individual::anonymous(blank())
}
pub fn annotations() -> AxiomAnnotations {
    AxiomAnnotations::new(vec![
        Annotation::new(iri("urn:outer"), AnnotationValue::Literal(literal())).with_annotations(
            AxiomAnnotations::new(vec![Annotation::new(
                iri("urn:inner"),
                AnnotationValue::AnonymousIndividual(blank()),
            )]),
        ),
    ])
}
