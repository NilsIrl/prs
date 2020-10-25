use clap::ArgMatches;

use super::Matcher;
use crate::cmd::arg::{ArgQuery, CmdArgOption};

/// The move command matcher.
pub struct MoveMatcher<'a> {
    matches: &'a ArgMatches<'a>,
}

impl<'a: 'b, 'b> MoveMatcher<'a> {
    /// The secret query.
    pub fn query(&self) -> Option<String> {
        ArgQuery::value(self.matches)
    }

    /// Secret target.
    pub fn target(&self) -> &str {
        self.matches.value_of("TARGET").unwrap()
    }
}

impl<'a> Matcher<'a> for MoveMatcher<'a> {
    fn with(matches: &'a ArgMatches) -> Option<Self> {
        matches
            .subcommand_matches("move")
            .map(|matches| MoveMatcher { matches })
    }
}