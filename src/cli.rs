use std::path::PathBuf;

use owo_colors::Style;

#[derive(clap::Parser, Clone, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Whether to color the output
    #[clap(long, global = true, default_value = "auto")]
    pub color: Color,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(clap::ValueEnum, Clone, Copy, Debug)]
pub enum Color {
    Always,
    Auto,
    Never,
}

impl Color {
    pub fn init(self) {
        // Set a supports-color override based on the variable passed in.
        match self {
            Color::Always => owo_colors::set_override(true),
            Color::Auto => {}
            Color::Never => owo_colors::set_override(false),
        }
    }

    pub fn should_color<T: std::io::IsTerminal>(self, term: &T) -> bool {
        match self {
            Color::Always => true,
            Color::Auto => term.is_terminal(),
            Color::Never => true,
        }
    }
}

#[derive(Debug, Default)]
pub struct Styles {
    pub citekey: Style,
    pub title: Style,
    pub authors: Style,
    pub venue: Style,
    pub year: Style,
    pub separator: Style,
    pub url: Style,
    pub bibtex_type: Style,
    pub bibtex_key: Style,
    pub bibtex_val: Style,
}

impl Styles {
    pub fn colorize(&mut self) {
        self.citekey = Style::new().cyan();
        self.title = Style::new().italic();
        self.authors = Style::new().magenta();
        self.venue = Style::new().green();
        self.year = Style::new().blue();
        self.separator = Style::new().bold();
        self.url = Style::new().underline();
        self.bibtex_type = Style::new().green().bold();
        self.bibtex_key = Style::new().magenta();
        self.bibtex_val = Style::new().blue();
    }
}

#[derive(clap::Subcommand, Debug, Clone)]
pub enum Commands {
    /// Search DBLP
    Search(#[command(flatten)] SearchArgs),
    /// Gets a bibtex entry from DBLP
    Get(#[command(flatten)] GetArgs),
    /// Fetches all DBLP bibtex entries for a LaTeX file
    GetAll(#[command(flatten)] GetAllArgs),
}

#[derive(clap::Args, Debug, Clone)]
pub struct SearchArgs {
    /// The DBLP query
    pub query: String,
    /// The type of query to perform
    #[arg(short, long, default_value_t = crate::dblp::search::Type::default())]
    pub r#type: crate::dblp::search::Type,
    /// The number of hits to request
    #[arg(short = 'n', long)]
    pub hits: Option<u32>,
}

#[derive(clap::Args, Debug, Clone, Copy)]
pub struct CommonGetArgs {
    /// Include unicode characters, rather than converting them to TeX
    #[arg(short, long)]
    pub unicode: bool,
}

#[derive(clap::Args, Debug, Clone)]
pub struct GetArgs {
    /// The DBLP citekey
    pub key: String,
    #[command(flatten)]
    pub common: CommonGetArgs,
}

#[derive(clap::Args, Debug, Clone)]
pub struct GetAllArgs {
    /// The LaTeX file to get all DBLP bibtex entries for
    ///
    /// The tool will read the `.aux` file. Assuming your main file is called `main.tex`, this can
    /// point to `main.tex`, `main.aux` or `main`.
    pub path: PathBuf,
    #[command(flatten)]
    pub common: CommonGetArgs,
    /// The maximum number of concurrent requests to DBLP to open
    #[arg(short = 'j', long, default_value_t = 8)]
    pub concurrent_requests: usize,
}
