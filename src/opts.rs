use crate::try_err::*;
use getopts::{HasArg, Occur, Options};
use std::fmt::Display;

pub enum Command {
    Filename,
    Hash,
    Urls(u64),
}

pub struct Opts {
    pub json: String,
    pub name: String,
    pub cmd: Command,
}

#[derive(Debug)]
pub enum OptError {
    OptError(getopts::Fail),
    String(String),
}

impl From<getopts::Fail> for OptError {
    fn from(v: getopts::Fail) -> Self {
        Self::OptError(v)
    }
}

impl From<&str> for OptError {
    fn from(v: &str) -> Self {
        Self::String(v.to_owned())
    }
}

impl Display for OptError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OptError(e) => e.fmt(f),
            Self::String(s) => s.fmt(f),
        }
    }
}

impl Opts {
    pub fn parse() -> Result<Self, OptError> {
        let mut opts = Options::new();
        opts.opt(
            "f",
            "file",
            "The path of the json file.",
            "PATH",
            HasArg::Yes,
            Occur::Req,
        );
        opts.opt(
            "n",
            "name",
            "The name of the dependency.",
            "NAME",
            HasArg::Yes,
            Occur::Req,
        );
        opts.optflag("F", "filename", "Return the file name.");
        opts.optflag("H", "hash", "Return the hash.");
        opts.optopt("u", "urls", "Return the index x of the urls", "x");
        opts.optflag("h", "help", "print this help menu");
        let args: Vec<String> = std::env::args().collect();
        if args.len() == 1 || (args.len() == 2 && (args[1] == "-h" || args[1] == "--help")) {
            let brief = format!("Usage: {} [options]", args[0]);
            print!("{}", opts.usage(&brief));
            std::process::exit(0);
        }
        let matches = opts.parse(&args[1..])?;
        let file = matches
            .opt_str("file")
            .try_err("Failed to get file name.")?;
        let name = matches
            .opt_str("name")
            .try_err("Failed to get dependency name.")?;
        let cmd = if matches.opt_present("filename") {
            Command::Filename
        } else if matches.opt_present("hash") {
            Command::Hash
        } else if matches.opt_present("urls") {
            let ind = matches.opt_str("urls").try_err("Failed to get index.")?;
            let ind = match ind.parse::<u64>() {
                Ok(ind) => ind,
                Err(e) => {
                    return Err(OptError::String(format!(
                        "Failed to get the index of url: {}",
                        e
                    )));
                }
            };
            Command::Urls(ind)
        } else {
            return Err(OptError::from("No action specified."));
        };
        Ok(Self {
            json: file,
            name,
            cmd,
        })
    }
}
