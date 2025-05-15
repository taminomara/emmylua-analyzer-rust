use std::str::FromStr;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct CmdArgs {
    #[structopt(
        parse(from_os_str),
        long = "input",
        short = "i",
        help = "The path of the lua project"
    )]
    pub input: std::path::PathBuf,

    #[structopt(
        default_value = "Markdown",
        long = "format",
        short = "f",
        help = "Format of the output, default is Markdown"
    )]
    pub format: Format,

    #[structopt(
        parse(from_os_str),
        default_value = "./output",
        long = "output",
        short = "o",
        help = "The output path of the docs file"
    )]
    pub output: std::path::PathBuf,

    #[structopt(
        parse(from_os_str),
        long = "override-template",
        help = "The path of the override template"
    )]
    pub override_template: Option<std::path::PathBuf>,

    #[structopt(
        parse(from_os_str),
        long = "mixin",
        help = "The path of the mixin md file"
    )]
    pub mixin: Option<std::path::PathBuf>,
}

#[derive(Debug, Eq, PartialEq, StructOpt)]
pub enum Format {
    Markdown,
    Json,
}

impl FromStr for Format {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "markdown" => Ok(Self::Markdown),
            "json" => Ok(Self::Json),
            _ => Err("Invalid format, must be one of markdown, json"),
        }
    }
}
