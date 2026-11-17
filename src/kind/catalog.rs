//! The catalog declaration shared by every structural kind enumeration.

/// Declare a kind enumeration whose variants each name one functional-syntax production.
macro_rules! structural_catalog {
    (
        $(#[$meta:meta])*
        $name:ident [$count:literal] {
            $($variant:ident => $production:literal),+ $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
        pub enum $name {
            $(
                #[doc = concat!("The `", $production, "` functional-syntax production.")]
                $variant,
            )+
        }

        impl $name {
            /// Every standard category, in declaration order.
            pub const ALL: [Self; $count] = [$(Self::$variant),+];

            /// Return the OWL 2 functional-syntax production this category represents.
            #[must_use]
            pub const fn functional_syntax_production(self) -> &'static str {
                match self {
                    $(Self::$variant => $production),+
                }
            }
        }
    };
}

pub(in crate::kind) use structural_catalog;
