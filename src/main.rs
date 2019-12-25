use regex::Regex;
use serde::Deserialize;
use std::path::PathBuf;
use structopt::StructOpt;

// https://github.com/actions/toolkit/blob/master/docs/problem-matchers.md#problem-matchers
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Matchers {
    problem_matcher: Vec<Matcher>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Matcher {
    owner: String,
    severity: Option<String>,
    pattern: Vec<Pattern>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Pattern {
    #[serde(with = "serde_regex")]
    regexp: Regex,
    file: Option<usize>,
    from_path: Option<usize>,
    line: Option<usize>,
    column: Option<usize>,
    severity: Option<usize>,
    code: Option<usize>,
    message: Option<usize>,
    #[serde(alias = "loop")]
    loops: Option<bool>,
}

#[derive(Debug, StructOpt)]
struct Opts {
    #[structopt(long, short)]
    /// Path of target text to inspect or "-" of reading from stdin
    target: String,
    /// List of paths that define problem matchers
    matchers: Vec<PathBuf>,
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn requires_args() {
        assert!(Opts::from_iter_safe(&["x"]).is_err());
    }

    #[test]
    fn parses_args() {
        assert!(Opts::from_iter_safe(&["x", "-t", "-"]).is_ok());
    }

    #[test]
    fn parses_single_line_matchers() {
        let matchers =
            serde_json::from_str::<Matchers>(include_str!("../tests/data/eslint-compact.json"));
        assert!(matchers.is_ok(), "matchers failed to parse: {:?}", matchers);
    }

    #[test]
    fn parses_multi_line_matchers() {
        let matchers =
            serde_json::from_str::<Matchers>(include_str!("../tests/data/eslint-stylish.json"));
        assert!(matchers.is_ok(), "matchers failed to parse: {:?}", matchers);
    }
}
