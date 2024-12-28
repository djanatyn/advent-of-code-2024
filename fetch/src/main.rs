//! Quick utility to fetch problem descriptions and input from advent of code.
//!
//! Requires a valid session cookie to authenticate.
//!
//! This script calls out to `pandoc` and `rdrview`, and will fail if those
//! utilities are not installed. You probably don't need to run this, it's just
//! for my convenience.

use duct::cmd;
use std::{env, fs};

pub const SESSION_COOKIE_ENV_VAR: &str = "AOC_SESSION_COOKIE";

/// Required application runtime configuration.
#[derive(Debug)]
pub struct Config {
    /// The path of the current problem, e.g. "2024/day/1".
    pub day: i64,
    /// A session cookie used to authenticate with Advent of Code.
    pub session_cookie: String,
}

impl Config {
    const BASE_URL: &str = "https://adventofcode.com/2024/day/";

    pub fn parse() -> Result<Self, String> {
        // parse cli args
        let day = match &env::args().collect::<Vec<String>>()[..] {
            [_, day] => day
                .parse::<i64>()
                .map_err(|e| format!("failed to convert date: {}", e))?,
            [cmd, ..] => Err(format!("usage: {cmd} <day>"))?,
            _ => Err("usage: fetch <day>")?,
        };
        // parse SESSION_COOKIE
        let session_cookie = match env::var(SESSION_COOKIE_ENV_VAR) {
            Ok(secret) => secret,
            Err(e) => Err(format!("please set {SESSION_COOKIE_ENV_VAR}: {e}"))?,
        };
        Ok(Config {
            day,
            session_cookie,
        })
    }

    fn description_url(&self) -> String {
        String::from(Self::BASE_URL) + format!("{:01}", &self.day).as_str()
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

    const BASE_PATH: &str = "solutions/src/bin";

    fn save(&self, config: &Config) -> Result<Success, String> {
        let problem_path = format!("{}/day{:02}.md", Self::BASE_PATH, &config.day);
        let input_path = format!("{}/day{:02}.input", Self::BASE_PATH, &config.day);

        let problem_markdown = cmd!("rdrview", "-H")
            .stdin_bytes(&*self.description)
            .pipe(cmd!("pandoc", "-f", "html", "-t", "gfm"))
            .read()
            .map_err(|e| e.to_string())?;

        fs::write(&problem_path, problem_markdown).map_err(|e| e.to_string())?;
        fs::write(&input_path, &self.input).map_err(|e| e.to_string())?;

        Ok(Success {
            problem_path,
            input_path,
        })
    }
}

#[derive(Debug)]
struct Success {
    problem_path: String,
    input_path: String,
}

fn main() -> Result<(), String> {
    let config = Config::parse()?;
    let problem =
        Problem::download(&config).map_err(|e| format!("failed to download problem: {}", e))?;
    let success = problem.save(&config)?;
    println!("saved problem to {}", success.problem_path);
    println!("saved input to {}", success.input_path);
    Ok(())
}
