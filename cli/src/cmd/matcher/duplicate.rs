use clap::ArgMatches;

use super::Matcher;
use crate::cmd::arg::{ArgQuery, CmdArgOption};

/// The duplicate command matcher.
pub struct DuplicateMatcher<'a> {
    matches: &'a ArgMatches<'a>,
}

impl<'a: 'b, 'b> DuplicateMatcher<'a> {
    /// The secret query.
    pub fn query(&self) -> Option<String> {
        ArgQuery::value(self.matches)
    }

    /// Secret target.
    pub fn target(&self) -> &str {
        self.matches.value_of("TARGET").unwrap()
    }
}

impl<'a> Matcher<'a> for DuplicateMatcher<'a> {
    fn with(matches: &'a ArgMatches) -> Option<Self> {
        matches
            .subcommand_matches("duplicate")
            .map(|matches| DuplicateMatcher { matches })
    }
}