//! Census of the vendored OWL 2 functional-syntax grammar against the kind catalogs.

use std::collections::{BTreeMap, BTreeSet};

use owl_structural_model::{AxiomKind, ClassExpressionKind, DataRangeKind};

const GRAMMAR: &str = include_str!("fixtures/owl2-functional-grammar.txt");
const COMPLETENESS: &str = include_str!("../COMPLETENESS.md");

/// The productions of the transcribed grammar, keyed by nonterminal name.
struct Grammar<'a> {
    // bounded: the 120 productions of one vendored, fixed-size specification appendix.
    productions: BTreeMap<&'a str, String>,
}

impl<'a> Grammar<'a> {
    fn parse(text: &'a str) -> Self {
        let mut productions = BTreeMap::new();
        let mut current: Option<&'a str> = None;
        for line in text.lines().filter(|line| !line.starts_with('#')) {
            if line.starts_with(char::is_whitespace) {
                let name = current.expect("continuation line follows a production");
                let rhs: &mut String = productions.get_mut(name).unwrap();
                rhs.push(' ');
                rhs.push_str(line.trim());
            } else if let Some((name, rhs)) = line.split_once(":=") {
                let name = name.trim();
                assert!(
                    productions.insert(name, rhs.trim().to_owned()).is_none(),
                    "duplicate production {name}"
                );
                current = Some(name);
            }
        }
        Self { productions }
    }

    /// Return the alternatives of a production that only chooses between nonterminals.
    fn choice(&self, name: &str) -> Option<Vec<&str>> {
        let rhs = self.productions.get(name)?;
        // bounded: alternatives of one grammar production.
        let alternatives: Vec<&str> = rhs.split('|').map(str::trim).collect();
        let is_choice = alternatives.len() > 1
            && alternatives
                .iter()
                .all(|alternative| self.productions.contains_key(alternative));
        is_choice.then_some(alternatives)
    }

    /// Expand nested choices into the concrete productions reachable from `root`.
    fn concrete_productions(&self, root: &str) -> Vec<&str> {
        let alternatives = self.choice(root).expect("root is a choice production");
        // bounded: concrete forms of one grammar category, at most 37.
        let mut leaves = Vec::new();
        for alternative in alternatives {
            if self.choice(alternative).is_some() {
                leaves.extend(self.concrete_productions(alternative));
            } else {
                leaves.push(alternative);
            }
        }
        leaves
    }

    fn assert_census(&self, root: &str, catalog: &[&'static str], expected: usize) {
        let leaves = self.concrete_productions(root);
        assert_eq!(leaves.len(), expected, "{root} productions: {leaves:?}");
        assert_eq!(catalog.len(), expected, "{root} kind catalog size");
        // bounded: one entry per concrete production of one category.
        let distinct: BTreeSet<&str> = leaves.iter().copied().collect();
        assert_eq!(distinct.len(), leaves.len(), "{root} repeats a production");
        for leaf in &leaves {
            let matches = catalog
                .iter()
                .filter(|production| *production == leaf)
                .count();
            assert_eq!(
                matches, 1,
                "{root} production {leaf} matched {matches} kinds"
            );
        }
        for production in catalog {
            assert!(
                distinct.contains(production),
                "{production} is not in {root}"
            );
        }
    }
}

fn productions<K: Copy>(all: &[K], production: fn(K) -> &'static str) -> Vec<&'static str> {
    all.iter().map(|kind| production(*kind)).collect()
}

/// Return the leading count written after `All ` in a COMPLETENESS.md table row.
fn documented_count(area: &str) -> usize {
    let row = COMPLETENESS
        .lines()
        .find(|line| line.starts_with(&format!("| {area} |")))
        .unwrap_or_else(|| panic!("COMPLETENESS.md has no {area} row"));
    let after = row
        .split_once("| All ")
        .unwrap_or_else(|| panic!("{area} row states no count"))
        .1;
    let digits: String = after.chars().take_while(char::is_ascii_digit).collect();
    digits.parse().unwrap()
}

#[test]
fn every_functional_syntax_production_matches_exactly_one_kind() {
    let grammar = Grammar::parse(GRAMMAR);
    grammar.assert_census(
        "Axiom",
        &productions(&AxiomKind::ALL, AxiomKind::functional_syntax_production),
        37,
    );
    grammar.assert_census(
        "ClassExpression",
        &productions(
            &ClassExpressionKind::ALL,
            ClassExpressionKind::functional_syntax_production,
        ),
        18,
    );
    grammar.assert_census(
        "DataRange",
        &productions(
            &DataRangeKind::ALL,
            DataRangeKind::functional_syntax_production,
        ),
        6,
    );
}

#[test]
fn completeness_counts_equal_the_kind_catalogs() {
    assert_eq!(documented_count("Axiom families"), AxiomKind::ALL.len());
    assert_eq!(
        documented_count("Class expressions"),
        ClassExpressionKind::ALL.len()
    );
    assert_eq!(documented_count("Data ranges"), DataRangeKind::ALL.len());
}
