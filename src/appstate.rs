use std::str::FromStr;

use anyhow::{Error, bail};
use clap::Parser;

#[derive(Debug, Clone, Copy)]
pub enum TraceStyle {
    None,
    Short,
    Full,
}

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

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, help="maximum number of squares to produce")]
    limit: Option<usize>,
    #[arg(short, long, default_value="none", help="trace style (none, short, full)")]
    trace: TraceStyle,
    #[arg(short, long, help="allow the same word two or more times in a square")]
    doubled: bool,
    #[arg(short, long, help="produce both canonical squares and their transposions")]
    transposed: bool,
}

pub struct AppState {
    pub nodes: usize,
    pub limit: Option<usize>,
    pub trace: TraceStyle,
    pub doubled: bool,
    pub transposed: bool,
}

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
    pub fn new() -> Self {
        let args = Args::parse();
        Self {
            nodes: 0,
            limit: args.limit,
            trace: args.trace,
            doubled: args.doubled,
            transposed: args.transposed,
        }
    }
}
