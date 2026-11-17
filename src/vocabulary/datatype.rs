//! The OWL 2 datatype map (section 4).

use super::catalog::vocabulary_catalog;

vocabulary_catalog! {
    /// A datatype of the OWL 2 datatype map.
    ///
    /// `rdfs:Literal` is a [`BuiltInEntity`](crate::BuiltInEntity), not a member of the map.
    OwlDatatype [33] {
        /// `owl:real` (section 4.1).
        Real => "http://www.w3.org/2002/07/owl#real",
        /// `owl:rational` (section 4.1).
        Rational => "http://www.w3.org/2002/07/owl#rational",
        /// `xsd:decimal` (section 4.1).
        Decimal => "http://www.w3.org/2001/XMLSchema#decimal",
        /// `xsd:integer` (section 4.1).
        Integer => "http://www.w3.org/2001/XMLSchema#integer",
        /// `xsd:nonNegativeInteger` (section 4.1).
        NonNegativeInteger => "http://www.w3.org/2001/XMLSchema#nonNegativeInteger",
        /// `xsd:nonPositiveInteger` (section 4.1).
        NonPositiveInteger => "http://www.w3.org/2001/XMLSchema#nonPositiveInteger",
        /// `xsd:positiveInteger` (section 4.1).
        PositiveInteger => "http://www.w3.org/2001/XMLSchema#positiveInteger",
        /// `xsd:negativeInteger` (section 4.1).
        NegativeInteger => "http://www.w3.org/2001/XMLSchema#negativeInteger",
        /// `xsd:long` (section 4.1).
        Long => "http://www.w3.org/2001/XMLSchema#long",
        /// `xsd:int` (section 4.1).
        Int => "http://www.w3.org/2001/XMLSchema#int",
        /// `xsd:short` (section 4.1).
        Short => "http://www.w3.org/2001/XMLSchema#short",
        /// `xsd:byte` (section 4.1).
        Byte => "http://www.w3.org/2001/XMLSchema#byte",
        /// `xsd:unsignedLong` (section 4.1).
        UnsignedLong => "http://www.w3.org/2001/XMLSchema#unsignedLong",
        /// `xsd:unsignedInt` (section 4.1).
        UnsignedInt => "http://www.w3.org/2001/XMLSchema#unsignedInt",
        /// `xsd:unsignedShort` (section 4.1).
        UnsignedShort => "http://www.w3.org/2001/XMLSchema#unsignedShort",
        /// `xsd:unsignedByte` (section 4.1).
        UnsignedByte => "http://www.w3.org/2001/XMLSchema#unsignedByte",
        /// `xsd:double` (section 4.2).
        Double => "http://www.w3.org/2001/XMLSchema#double",
        /// `xsd:float` (section 4.2).
        Float => "http://www.w3.org/2001/XMLSchema#float",
        /// `rdf:PlainLiteral` (section 4.3).
        PlainLiteral => "http://www.w3.org/1999/02/22-rdf-syntax-ns#PlainLiteral",
        /// `xsd:string` (section 4.3).
        String => "http://www.w3.org/2001/XMLSchema#string",
        /// `xsd:normalizedString` (section 4.3).
        NormalizedString => "http://www.w3.org/2001/XMLSchema#normalizedString",
        /// `xsd:token` (section 4.3).
        Token => "http://www.w3.org/2001/XMLSchema#token",
        /// `xsd:language` (section 4.3).
        Language => "http://www.w3.org/2001/XMLSchema#language",
        /// `xsd:Name` (section 4.3).
        Name => "http://www.w3.org/2001/XMLSchema#Name",
        /// `xsd:NCName` (section 4.3).
        NcName => "http://www.w3.org/2001/XMLSchema#NCName",
        /// `xsd:NMTOKEN` (section 4.3).
        Nmtoken => "http://www.w3.org/2001/XMLSchema#NMTOKEN",
        /// `xsd:boolean` (section 4.4).
        Boolean => "http://www.w3.org/2001/XMLSchema#boolean",
        /// `xsd:hexBinary` (section 4.5).
        HexBinary => "http://www.w3.org/2001/XMLSchema#hexBinary",
        /// `xsd:base64Binary` (section 4.5).
        Base64Binary => "http://www.w3.org/2001/XMLSchema#base64Binary",
        /// `xsd:anyURI` (section 4.6).
        AnyUri => "http://www.w3.org/2001/XMLSchema#anyURI",
        /// `xsd:dateTime` (section 4.7).
        DateTime => "http://www.w3.org/2001/XMLSchema#dateTime",
        /// `xsd:dateTimeStamp` (section 4.7).
        DateTimeStamp => "http://www.w3.org/2001/XMLSchema#dateTimeStamp",
        /// `rdf:XMLLiteral` (section 4.8).
        XmlLiteral => "http://www.w3.org/1999/02/22-rdf-syntax-ns#XMLLiteral",
    }
}
