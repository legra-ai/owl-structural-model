//! Built-in entities of the OWL 2 structural specification (section 5).

use super::catalog::vocabulary_catalog;
use crate::{Entity, EntityKind};

vocabulary_catalog! {
    /// An entity with a predefined meaning in every OWL 2 ontology.
    BuiltInEntity [7] {
        /// `owl:Thing`, the class containing every individual.
        Thing => "http://www.w3.org/2002/07/owl#Thing",
        /// `owl:Nothing`, the empty class.
        Nothing => "http://www.w3.org/2002/07/owl#Nothing",
        /// `owl:topObjectProperty`, relating every pair of individuals.
        TopObjectProperty => "http://www.w3.org/2002/07/owl#topObjectProperty",
        /// `owl:bottomObjectProperty`, relating no individuals.
        BottomObjectProperty => "http://www.w3.org/2002/07/owl#bottomObjectProperty",
        /// `owl:topDataProperty`, relating every individual to every literal.
        TopDataProperty => "http://www.w3.org/2002/07/owl#topDataProperty",
        /// `owl:bottomDataProperty`, relating no individual to any literal.
        BottomDataProperty => "http://www.w3.org/2002/07/owl#bottomDataProperty",
        /// `rdfs:Literal`, the datatype containing every data value.
        Literal => "http://www.w3.org/2000/01/rdf-schema#Literal",
    }
}

impl BuiltInEntity {
    /// Return the entity category this built-in term belongs to.
    #[must_use]
    pub const fn entity_kind(self) -> EntityKind {
        match self {
            Self::Thing | Self::Nothing => EntityKind::Class,
            Self::TopObjectProperty | Self::BottomObjectProperty => EntityKind::ObjectProperty,
            Self::TopDataProperty | Self::BottomDataProperty => EntityKind::DataProperty,
            Self::Literal => EntityKind::Datatype,
        }
    }

    /// Return this built-in term as a typed entity.
    #[must_use]
    pub fn entity(self) -> Entity {
        Entity::new(self.entity_kind(), self.into())
    }
}
