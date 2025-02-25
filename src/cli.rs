use owo_colors::{OwoColorize, Style};

#[derive(clap::Parser, Clone, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The DBLP query
    pub query: String,
    /// The type of query to perform
    #[arg(short, long, default_value_t = crate::dblp::query::Type::default())]
    pub r#type: crate::dblp::query::Type,
    /// The number of hits to request
    #[arg(short = 'n', long)]
    pub hits: Option<u32>,
    /// Whether to color the output
    #[clap(long, global = true, default_value = "auto")]
    pub color: Color,
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
    }
}
