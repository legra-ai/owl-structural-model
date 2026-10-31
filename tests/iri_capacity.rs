//! Public OWL and shared RDF values use one capacity-reporting IRI type.

use iri_rfc3987::Iri;
use owl_structural_model::{OntologyId, OwlLiteral};

#[test]
fn ontology_and_literal_iris_expose_their_retained_capacity() {
    let mut ontology_text = String::with_capacity(4096);
    ontology_text.push_str("urn:ontology:allocated");
    let ontology_capacity = ontology_text.capacity();
    let ontology = OntologyId::named(Iri::parse_owned(ontology_text).unwrap());
    let ontology_iri: &Iri<String> = ontology.ontology_iri().unwrap();
    assert_eq!(ontology_iri.capacity(), ontology_capacity);

    let mut datatype_text = String::with_capacity(8192);
    datatype_text.push_str("urn:datatype:allocated");
    let datatype_capacity = datatype_text.capacity();
    let literal = OwlLiteral::typed(
        "exact lexical form",
        Iri::parse_owned(datatype_text).unwrap(),
    )
    .unwrap();
    let datatype: &Iri<String> = literal.metadata().datatype();
    assert_eq!(datatype.capacity(), datatype_capacity);
}
