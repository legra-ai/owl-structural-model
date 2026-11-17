//! The declaration shared by every closed vocabulary catalog.

/// Declare a documented closed catalog of vocabulary terms with their full IRIs.
macro_rules! vocabulary_catalog {
    (
        $(#[$meta:meta])*
        $name:ident [$count:literal] {
            $($(#[$variant_meta:meta])* $variant:ident => $iri:literal),+ $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub enum $name {
            $($(#[$variant_meta])* $variant,)+
        }

        impl $name {
            /// Every term of this catalog, in specification order.
            pub const ALL: [Self; $count] = [$(Self::$variant),+];

            /// Return the full IRI text of this term.
            #[must_use]
            pub const fn as_str(self) -> &'static str {
                match self {
                    $(Self::$variant => $iri),+
                }
            }
        }

        impl $crate::vocabulary::VocabularyTerm for $name {
            fn catalog() -> &'static [Self] {
                &Self::ALL
            }

            fn as_str(self) -> &'static str {
                Self::as_str(self)
            }
        }

        impl ::core::fmt::Display for $name {
            fn fmt(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                formatter.write_str(self.as_str())
            }
        }

        impl From<$name> for ::iri_rfc3987::Iri<String> {
            fn from(term: $name) -> Self {
                $crate::vocabulary::VocabularyTerm::iri(term)
            }
        }
    };
}

pub(in crate::vocabulary) use vocabulary_catalog;
