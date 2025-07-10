//! Application state structure used in the search. Also,
//! command-line argument parsing.

use std::str::FromStr;
use std::path::PathBuf;

use anyhow::{Error, bail};
use clap::Parser;

/// Used for progress traces.
#[derive(Debug, Clone, Copy)]
pub enum TraceStyle {
    None,
    Short,
    Full,
}

/// The argument parser needs to know names for the trace
/// styles.
impl FromStr for TraceStyle {
    type Err = Error;

    fn from_str(style: &str) -> Result<Self, Error> {
        match style {
            "none" => Ok(TraceStyle::None),
            "short" => Ok(TraceStyle::Short),
            "full" => Ok(TraceStyle::Full),
            s => bail!("{s}: unknown trace style"),
        }
    }
}

/// The command-line argument struct.
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, help="maximum number of squares to produce")]
    limit: Option<usize>,
    #[arg(
        short='p',
        long="progress",
        default_value="none",
        help="progress trace style (none, short, full)",
    )]
    trace: TraceStyle,
    #[arg(short, long, help="allow the same word two or more times in a square")]
    doubled: bool,
    #[arg(short, long, help="produce both canonical squares and their transposions")]
    transposed: bool,
    #[arg(short, long, default_value="strings.json", help="output file")]
    pub output: PathBuf,
    #[arg(help="dictionary", default_value="usa_5.txt")]
    pub dict: PathBuf,
}

/// The app state. Contains things needed during search.
pub struct AppState {
    /// Number of nodes searched.
    pub nodes: usize,
    /// Potential limit on solutions returned.
    pub limit: Option<usize>,
    /// Progress trace style.
    pub trace: TraceStyle,
    /// Allow words to appear multiple times in a square.
    pub doubled: bool,
    /// Allow transposed squares.
    pub transposed: bool,
}

#[cfg(test)]
impl Default for AppState {
    fn default() -> Self {
        Self {
            nodes: 0,
            limit: Some(1000),
            trace: TraceStyle::None,
            doubled: false,
            transposed: false,
        }
    }
}

impl AppState {
    /// Given the arguments, set up the state.
    pub fn new(args: &Args) -> Self {
        Self {
            nodes: 0,
            limit: args.limit,
            trace: args.trace,
            doubled: args.doubled,
            transposed: args.transposed,
        }
    }
}
