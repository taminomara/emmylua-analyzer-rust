use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct CmdArgs {
    /// The path of the lua file
    #[structopt(
        parse(from_os_str),
        long = "input",
        short = "i",
        help = "The path of the lua project"
    )]
    pub input: std::path::PathBuf,
    /// The output path of the markdown file
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
