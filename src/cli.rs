use owo_colors::Style;

#[derive(clap::Parser, Clone, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Whether to color the output
    #[arg(long, global = true, default_value = "auto")]
    pub color: Color,
    #[command(subcommand)]
    pub command: Commands,
    #[command(flatten)]
    pub dblp: DblpServerArgs,
}

/// Arguments related to the DBLP server to use
#[derive(clap::Args, Clone, Debug)]
#[command(next_help_heading = "DBLP server args")]
pub struct DblpServerArgs {
    /// Use the server at `dblp.uni-trier.de`, rather than `dblp.org`
    #[arg(short, long, global = true)]
    pub trier: bool,
    /// Use a custom DBLP server
    #[arg(long, global = true, conflicts_with = "trier")]
    pub dblp_domain: Option<String>,
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
    pub info: Style,
    pub warn_head: Style,
    pub warn_body: Style,
}

impl Styles {
    pub const GET_ALL_PROGRESS_TEMPLATE: &'static str =
        "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}";

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
        self.info = Style::new().cyan().bold();
        self.warn_head = Style::new().yellow().bold();
        self.warn_body = Style::new().yellow();
    }
}

macro_rules! warning {
    ($context:expr, $($arg:tt)*) => {{
        use std::io::IsTerminal;

        use owo_colors::OwoColorize;

        let mut styles = crate::cli::Styles::default();
        // TODO: ideally we should respect the `color` option here
        if std::io::stderr().is_terminal() {
            styles.colorize();
        }
        eprint!(
            "{}{}{} ",
            "Warning (".style(styles.warn_head),
            $context.style(styles.warn_head),
            "):".style(styles.warn_head)
        );
        eprintln!("{}", format!($($arg)*).style(styles.warn_body));
    }};
}
pub(crate) use warning;

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
    #[arg(short = 'T', long, default_value_t = crate::dblp::search::Type::default())]
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
    /// Whether to use crossref style citations for `incollections` and `inproceedings`
    #[arg(short, long)]
    pub crossref: bool,
    /// Keep the all external fields (DOI and URL), rather than just one
    #[arg(long)]
    pub all_externals: bool,
    /// Don't expand journal abbreviations
    #[arg(long)]
    pub dont_expand_journals: bool,
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
    pub path: camino::Utf8PathBuf,
    #[command(flatten)]
    pub common: CommonGetArgs,
    /// The maximum number of concurrent requests to DBLP to open
    #[arg(short = 'j', long, default_value_t = 8)]
    pub concurrent_requests: usize,
    /// Don't follow `\@input` commands in the LaTeX aux file
    #[arg(short = 'f', long)]
    pub no_follow_inputs: bool,
}

#[cfg(test)]
mod tests {
    #[test]
    fn verify_cli_args() {
        use clap::CommandFactory;
        super::Args::command().debug_assert()
    }
}
