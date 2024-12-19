//! Quick utility to fetch problem descriptions and input from advent of code.
//!
//! Requires a valid session cookie to authenticate.
//!
//! This script calls out to `pandoc` and `rdrview`, and will fail if those
//! utilities are not installed. You probably don't need to run this, it's just
//! for my convenience.

use std::env;

/// Required application runtime configuration.
#[derive(Debug)]
pub struct Config {
    /// The path of the current problem, e.g. "2024/day/1"
    pub problem: String,
    /// A session cookie used to authenticate with Advent of Code.
    pub session_cookie: String,
}

impl Config {
    pub fn parse() -> Result<Self, String> {
        // parse cli args
        let problem = match &env::args().collect::<Vec<String>>()[..] {
            [_, problem] => problem.clone(),
            [cmd, ..] => Err(format!("usage: {cmd} <YYYY/day/N>"))?,
            _ => Err("usage: fetch <YYYY/day/N>")?,
        };
        // parse SESSION_COOKIE
        let session_cookie = match env::var("SESSION_COOKIE") {
            Ok(secret) => secret,
            Err(e) => Err(format!("please set SESSION_COOKIE: {e}"))?,
        };
        Ok(Config {
            problem,
            session_cookie,
        })
    }
}

fn main() -> Result<(), String> {
    let _config = Config::parse()?;
    Ok(())
}
