//! Deserialization enforces constructor invariants in text and binary formats.

use iri_rfc3987::Iri;
use owl_structural_model::{OntologyId, OwlLiteral};
type ClassExpression = owl_structural_model::ClassExpression;
type AnnotationValue = owl_structural_model::AnnotationValue;
type Axiom = owl_structural_model::Axiom;
type AxiomAnnotations = owl_structural_model::AxiomAnnotations;
type OntologyPatch = owl_structural_model::OntologyPatch;
type OntologyPatchOperation = owl_structural_model::OntologyPatchOperation;
use rdf_structural_model::RDF_LANG_STRING;
use serde::ser::SerializeStructVariant;
use serde::{Serialize, Serializer, de::DeserializeOwned};

fn iri(value: &str) -> Iri<String> {
    Iri::parse(value).unwrap().into()
}

fn assert_rejected<T: DeserializeOwned, S: Serialize>(invalid: &S) {
    let json = serde_json::to_string(invalid).unwrap();
    let bytes = postcard::to_allocvec(invalid).unwrap();
    assert!(serde_json::from_str::<T>(&json).is_err(), "accepted {json}");
    assert!(
        postcard::from_bytes::<T>(&bytes).is_err(),
        "accepted binary input"
    );
}

#[derive(Serialize)]
struct IdentityInput {
    ontology_iri: Option<Iri<String>>,
    version_iri: Option<Iri<String>>,
}

#[derive(Serialize)]
struct PatchInput {
    target: OntologyId,
    operations: Vec<OntologyPatchOperation>,
}

#[derive(Serialize)]
struct LiteralInput {
    lexical: String,
    metadata: MetadataInput,
}

#[derive(Serialize)]
struct MetadataInput {
    datatype: Iri<String>,
    facets: FacetsInput,
}

#[derive(Serialize)]
enum FacetsInput {
    None,
    Language(String),
}

impl LiteralInput {
    fn language(language: &str) -> Self {
        Self {
            lexical: "hello".to_owned(),
            metadata: MetadataInput {
                datatype: iri(RDF_LANG_STRING),
                facets: FacetsInput::Language(language.to_owned()),
            },
        }
    }
}

#[derive(Serialize)]
struct BlankInput {
    scope: Iri<String>,
    label: String,
}

#[test]
fn version_only_identity_is_rejected_in_both_formats() {
    assert_rejected::<OntologyId, _>(&IdentityInput {
        ontology_iri: None,
        version_iri: Some(iri("urn:version")),
    });
}

#[test]
fn empty_patch_is_rejected_in_both_formats() {
    assert_rejected::<OntologyPatch, _>(&PatchInput {
        target: OntologyId::anonymous(),
        operations: vec![],
    });
}

#[test]
fn empty_language_tag_is_rejected_in_both_formats() {
    assert_rejected::<OwlLiteral, _>(&LiteralInput::language(""));
}

#[test]
fn empty_anonymous_annotation_is_rejected_in_both_formats() {
    assert_rejected::<AnnotationValue, _>(
        &owl_structural_model::AnnotationValue::<OwlLiteral, _>::AnonymousIndividual(BlankInput {
            scope: iri("urn:document"),
            label: String::new(),
        }),
    );
}

#[test]
fn nested_literal_validation_survives_patch_deserialization() {
    let patch = owl_structural_model::OntologyPatch::<LiteralInput>::try_new(
        OntologyId::anonymous(),
        vec![
            owl_structural_model::OntologyPatchOperation::AddAnnotation {
                annotation: owl_structural_model::Annotation::new(
                    iri("urn:label"),
                    owl_structural_model::AnnotationValue::Literal(LiteralInput::language("")),
                ),
            },
        ],
    )
    .unwrap();
    assert_rejected::<OntologyPatch, _>(&patch);
}

struct ClassOperandsInput {
    union: bool,
    operands: Vec<ClassExpression>,
}
impl Serialize for ClassOperandsInput {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let (index, name) = if self.union {
            (2, "ObjectUnionOf")
        } else {
            (1, "ObjectIntersectionOf")
        };
        serializer.serialize_newtype_variant("ClassExpression", index, name, &self.operands)
    }
}
struct AxiomOperandsInput {
    disjoint: bool,
    classes: Vec<ClassExpression>,
}
impl Serialize for AxiomOperandsInput {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let (index, name) = if self.disjoint {
            (3, "DisjointClasses")
        } else {
            (2, "EquivalentClasses")
        };
        let mut value = serializer.serialize_struct_variant("Axiom", index, name, 2)?;
        value.serialize_field("classes", &self.classes)?;
        value.serialize_field("annotations", &AxiomAnnotations::empty())?;
        value.end()
    }
}
#[test]
fn empty_class_operands_are_rejected_in_both_formats() {
    // bounded: the two invalid occurrence counts and two Boolean forms.
    for operands in [vec![], vec![ClassExpression::thing()]] {
        for union in [false, true] {
            assert_rejected::<ClassExpression, _>(&ClassOperandsInput {
                union,
                operands: operands.clone(),
            });
        }
    }
}
#[test]
fn short_class_axioms_are_rejected_in_both_formats() {
    // bounded: the two invalid occurrence counts and two class-axiom forms.
    for classes in [vec![], vec![ClassExpression::thing()]] {
        for disjoint in [false, true] {
            assert_rejected::<Axiom, _>(&AxiomOperandsInput {
                disjoint,
                classes: classes.clone(),
            });
        }
    }
}

#[test]
fn malformed_language_tags_are_rejected_in_both_formats() {
    // bounded: representative malformed syntax and duplicate subtag cases.
    for language in ["en--US", "en_US", "de-1901-1901", "en-a-abc-A-def"] {
        assert_rejected::<OwlLiteral, _>(&LiteralInput::language(language));
    }
}

#[test]
fn language_tag_case_does_not_change_literal_identity() {
    assert_eq!(
        OwlLiteral::language_tagged(
            "hello",
            rdf_structural_model::LanguageTag::try_new("en-US".to_owned()).unwrap()
        ),
        OwlLiteral::language_tagged(
            "hello",
            rdf_structural_model::LanguageTag::try_new("EN-us".to_owned()).unwrap()
        ),
    );
}

#[test]
fn language_datatype_requires_language_facets() {
    assert!(OwlLiteral::typed("hello", iri(RDF_LANG_STRING)).is_err());
    assert_rejected::<OwlLiteral, _>(&LiteralInput {
        lexical: "hello".to_owned(),
        metadata: MetadataInput {
            datatype: iri(RDF_LANG_STRING),
            facets: FacetsInput::None,
        },
    });
}
