use structopt::StructOpt;

#[allow(unused)]
#[derive(Debug, StructOpt, Clone)]
#[structopt(name = "emmylua-check", about = "EmmyLua Check")]
pub struct CmdArgs {
    /// Specify configuration file path.
    /// If not provided, both ".emmyrc.json" and ".luarc.json" will be searched in the workspace
    /// directory
    #[structopt(short, long, parse(from_os_str))]
    pub config: Option<std::path::PathBuf>,

    /// Path to the workspace directory
    #[structopt(parse(from_os_str))]
    pub workspace: std::path::PathBuf,

    /// Comma separated list of ignore patterns.
    /// Patterns must follow glob syntax
    #[structopt(short, long, use_delimiter = true)]
    pub ignore: Option<Vec<String>>,

    /// Specify output format
    #[structopt(
        long,
        default_value = "text",
        possible_values = &OutputFormat::variants(),
        case_insensitive = true
    )]
    pub output_format: OutputFormat,

    /// Specify output destination (stdout or a file path, only used when output_format is json).
    #[structopt(long, default_value = "stdout", parse(try_from_str))]
    pub output: OutputDestination,

    /// Treat warnings as errors
    #[structopt(long)]
    pub warnings_as_errors: bool,

    /// Verbose output
    #[structopt(long)]
    pub verbose: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OutputFormat {
    Json,
    Text,
}

impl std::str::FromStr for OutputFormat {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "text" => Ok(OutputFormat::Text),
            _ => Err(format!("Invalid output format: {}", s)),
        }
    }
}

impl OutputFormat {
    pub fn variants() -> [&'static str; 2] {
        ["json", "text"]
    }
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum OutputDestination {
    Stdout,
    File(std::path::PathBuf),
}

impl std::str::FromStr for OutputDestination {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "stdout" => Ok(OutputDestination::Stdout),
            _ => Ok(OutputDestination::File(std::path::PathBuf::from(s))),
        }
    }
}
