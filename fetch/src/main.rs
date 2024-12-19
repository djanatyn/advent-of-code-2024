//! Quick utility to fetch problem descriptions and input from advent of code.
//!
//! Requires a valid session cookie to authenticate.
//!
//! This script calls out to `pandoc` and `rdrview`, and will fail if those
//! utilities are not installed. You probably don't need to run this, it's just
//! for my convenience.

use std::env;

pub const SESSION_COOKIE_ENV_VAR: &str = "AOC_SESSION_COOKIE";

/// Required application runtime configuration.
#[derive(Debug)]
pub struct Config {
    /// The path of the current problem, e.g. "2024/day/1".
    pub path: String,
    /// A session cookie used to authenticate with Advent of Code.
    pub session_cookie: String,
}

impl Config {
    const BASE_URL: &str = "https://adventofcode.com/";

    pub fn parse() -> Result<Self, String> {
        // parse cli args
        let path = match &env::args().collect::<Vec<String>>()[..] {
            [_, problem] => problem.clone(),
            [cmd, ..] => Err(format!("usage: {cmd} <YYYY/day/N>"))?,
            _ => Err("usage: fetch <YYYY/day/N>")?,
        };
        // parse SESSION_COOKIE
        let session_cookie = match env::var(SESSION_COOKIE_ENV_VAR) {
            Ok(secret) => secret,
            Err(e) => Err(format!("please set {SESSION_COOKIE_ENV_VAR}: {e}"))?,
        };
        Ok(Config {
            path,
            session_cookie,
        })
    }

    fn description_url(&self) -> String {
        String::from(Self::BASE_URL) + &self.path
    }

    fn input_url(&self) -> String {
        self.description_url() + "/input"
    }
}

/// Problem input and problem description (as HTML)
#[derive(Debug)]
pub struct Problem {
    /// Description (HTML). Possibly includes part 2.
    description: String,
    /// Problem input (text).
    input: String,
}

impl Problem {
    fn request(config: &Config, path: String) -> Result<String, ureq::Error> {
        Ok(ureq::get(&path)
            .set(
                "Cookie",
                format!("session={}", config.session_cookie).as_str(),
            )
            .call()?
            .into_string()?)
    }

    pub fn download(config: &Config) -> Result<Self, ureq::Error> {
        let description = Self::request(config, config.description_url())?;
        let input = Self::request(config, config.input_url())?;
        Ok(Problem { description, input })
    }
}

fn main() -> Result<(), String> {
    // let _problem = Problem::download(&Config::parse()?);
    Ok(())
}
