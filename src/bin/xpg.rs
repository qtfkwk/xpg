use anyhow::Result;
use clap::{Parser, ValueEnum, builder::Styles};
use convert_case::Casing;
use std::path::PathBuf;

#[cfg(unix)]
use pager::Pager;

const VERSION: &str = env!("CARGO_PKG_VERSION");

const STYLES: Styles = Styles::styled()
    .header(clap_cargo::style::HEADER)
    .usage(clap_cargo::style::USAGE)
    .literal(clap_cargo::style::LITERAL)
    .placeholder(clap_cargo::style::PLACEHOLDER)
    .error(clap_cargo::style::ERROR)
    .valid(clap_cargo::style::VALID)
    .invalid(clap_cargo::style::INVALID);

#[derive(Parser)]
#[command(
    about,
    version,
    max_term_width = 80,
    styles = STYLES,
    before_help = format!("xpg {VERSION} <https://crates.io/crates/xpg>"),
    after_help = "\
---

Notes

1. Patterns
    * Words
        * `W`: WORD
        * `w`: word
        * `T`: Word
        * `k`: `shuffle(cccc(c|d)(C|c))`
        * `{sub}`: see `WordKind`s section in readme
    * Characters
        * `C`: A-Z
        * `c`: a-z
        * `d`: 0-9
        * `s`: `~!@#$%^&*-_=+;:,./?()[]{}<>`
        * `a`: `C` | `c` | `d` | `s`
    * Special
        * `keychain`
        * `codename`
        * `codename-series`
        * `haiku--` | `haiku-condensed`
        * `haiku-` | `haiku-with-syllables-condensed`
        * `haiku`
        * `haiku+` | `haiku-with-syllables`
        * `haiku++` | `haiku-full`
        * `haiku+++` | `haiku-full-with-syllables`

2. Ten attempts are made to satisfy the given length requirements by default;
   use `-a N` and set `N` to the number of attempts to increase chances.
   It will exit with an error if unable to satisfy requirements; some
   configurations will always produce an error.

3. Extended words are only available in exclusively extended word kinds by
   default; however, with `-e`, they are merged with regular word kinds; use
   with the `-L` option to see its effect.
"
)]
struct Cli {
    /// Shuffle characters
    #[arg(short, long)]
    shuffle: bool,

    /// Number of passwords (0:∞ )
    #[arg(short, long, value_name = "NUMBER", default_value = "1")]
    count: usize,

    /// Length
    #[arg(short, long, value_name = "NUMBER", conflicts_with_all = ["minimum", "maximum"])]
    length: Option<usize>,

    /// Minimum length
    #[arg(short = 'm', long, value_name = "NUMBER")]
    minimum: Option<usize>,

    /// Maximum length
    #[arg(short = 'M', long, value_name = "NUMBER")]
    maximum: Option<usize>,

    /// Attempts (see note 2)
    #[arg(short, long, value_name = "NUMBER", default_value = "10")]
    attempts: usize,

    /// Configuration file
    #[arg(short = 'C', long, value_name = "PATH")]
    config: Option<PathBuf>,

    /// Merge extended words (see note 3)
    #[arg(short, long)]
    extended: bool,

    /// Apply case style
    #[arg(short = 'A', long, value_name = "STYLE")]
    apply_case: Option<Case>,

    /// List words in `{sub}`(s)
    #[arg(short = 'L', long)]
    list: bool,

    /// Print configuration
    #[arg(short, long, conflicts_with = "list")]
    dump_config: bool,

    /// Print readme
    #[arg(short, long)]
    readme: bool,

    /// Pattern(s) (see note 1) [default: `wwww`, `-L`: `{a}`]
    patterns: Vec<String>,
}

impl Cli {
    fn process_config(&self) -> Result<xpg::Config> {
        if let Some(config) = &self.config {
            // Load a custom configuration via -C
            return xpg::Config::from_path(config, self.extended);
        } else if let Some(proj_dirs) = directories::ProjectDirs::from("com", "qtfkwk", "xpg") {
            // Load a custom configuration file
            let config_dir = proj_dirs.config_dir();
            let user_config = config_dir.join("config.json");
            if user_config.exists() {
                return xpg::Config::from_path(&user_config, self.extended);
            }
        }

        // Load the default configuration
        xpg::Config::from_str(xpg::CONFIG, self.extended)
    }
}

/**
Provides the command line interface
*/
fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.readme {
        #[cfg(unix)]
        Pager::with_pager("bat -pl md").setup();

        print!("{}", include_str!("../../README.md"));
        std::process::exit(0);
    }

    // Process the configuration
    let cfg = cli.process_config()?;

    if cli.dump_config {
        println!("{}", cfg.dump()?);
        std::process::exit(0);
    } else if cli.list {
        let patterns = if cli.patterns.is_empty() {
            if cli.extended {
                vec![String::from("{a.ext}")]
            } else {
                vec![String::from("{a}")]
            }
        } else {
            cli.patterns.clone()
        };
        for sub in &patterns {
            for word in &cfg.list(sub)? {
                println!("{word}");
            }
        }
        std::process::exit(0);
    }

    // Convert --length, --minimum, --maximum option values into a range
    let length = if let Some(n) = cli.length {
        n..=n
    } else if let Some(n) = cli.minimum {
        if let Some(m) = cli.maximum {
            n..=m
        } else {
            n..=usize::MAX
        }
    } else if let Some(m) = cli.maximum {
        0..=m
    } else {
        0..=usize::MAX
    };

    let patterns = if cli.patterns.is_empty() {
        vec![String::from("wwww")]
    } else {
        cli.patterns.clone()
    };

    for pattern in &patterns {
        // Pad pattern to the minimum length with last character of pattern if it doesn't have any words
        let min = length.start();
        let len = pattern.len();
        let d = if len < *min { min - len } else { 0 };
        let pattern = if d > 0
            && ![
                "keychain",
                "codename",
                "codename-series",
                "haiku",
                "haiku+",
                "haiku-",
                "haiku--",
                "haiku++",
                "haiku+++",
                "haiku-with-syllables",
                "haiku-with-syllables-condensed",
                "haiku-condensed",
                "haiku-full",
                "haiku-full-with-syllables",
            ]
            .contains(&pattern.as_str())
            && !pattern.contains(['w', 'W', 'T'])
            && !cfg.subs.keys().any(|x| pattern.contains(x))
        {
            format!(
                "{}{}",
                pattern,
                format!("{}", pattern.chars().last().unwrap()).repeat(d),
            )
        } else {
            pattern.clone()
        };

        // Adjust meaning of -c for codename-series
        let (count, codenames) = if pattern == "codename-series" {
            (1, if cli.count == 1 { 10 } else { cli.count })
        } else {
            (cli.count, 0)
        };

        // Generate password(s)
        let mut i = 0;
        loop {
            let mut attempts = 0;
            loop {
                let pw = match pattern.as_str() {
                    "keychain" => cfg.keychain(),
                    "codename" => cfg.codename(),
                    "codename-series" => cfg.codename_series(codenames),
                    "haiku" => cfg.haiku(xpg::HaikuVariant::Normal),
                    "haiku+" | "haiku-with-syllables" => {
                        cfg.haiku(xpg::HaikuVariant::WithSyllables)
                    }
                    "haiku-" | "haiku-with-syllables-condensed" => {
                        cfg.haiku(xpg::HaikuVariant::WithSyllablesCondensed)
                    }
                    "haiku--" | "haiku-condensed" => cfg.haiku(xpg::HaikuVariant::Condensed),
                    "haiku++" | "haiku-full" => cfg.haiku(xpg::HaikuVariant::Full),
                    "haiku+++" | "haiku-full-with-syllables" => {
                        cfg.haiku(xpg::HaikuVariant::FullWithSyllables)
                    }
                    _ => cfg.generate(&pattern),
                };

                // Enforce length requirements
                if length.contains(&pw.len()) {
                    // Apply case style
                    let pw = if let Some(case) = &cli.apply_case {
                        case.apply(&pw)
                    } else {
                        pw
                    };

                    // Shuffle characters
                    let pw = if cli.shuffle { xpg::shuffle(&pw) } else { pw };

                    println!("{pw}");
                    break;
                } else {
                    attempts += 1;
                    if attempts >= cli.attempts {
                        error("Maximum attempts exceeded!");
                    }
                }
            }
            i += 1;
            if count > 0 && i >= count {
                break;
            }
        }
    }

    Ok(())
}

fn error(msg: &str) {
    eprintln!("Error: {msg}");
    std::process::exit(1);
}

#[derive(Clone, ValueEnum)]
enum Case {
    Upper,
    Lower,
    Title,
    Toggle,
    Camel,
    Pascal,
    UpperCamel,
    Snake,
    Constant,
    UpperSnake,
    ScreamingSnake,
    Kebab,
    Cobol,
    UpperKebab,
    Train,
    Flat,
    UpperFlat,
    Alternating,
    Random,
    PseudoRandom,
    Sentence,
}

impl Case {
    fn apply(&self, s: &str) -> String {
        s.to_case(match self {
            Case::Upper => convert_case::Case::Upper,
            Case::Lower => convert_case::Case::Lower,
            Case::Title => convert_case::Case::Title,
            Case::Sentence => convert_case::Case::Sentence,
            Case::Toggle => convert_case::Case::Toggle,
            Case::Camel => convert_case::Case::Camel,
            Case::Pascal => convert_case::Case::Pascal,
            Case::UpperCamel => convert_case::Case::UpperCamel,
            Case::Snake => convert_case::Case::Snake,
            Case::Constant => convert_case::Case::Constant,
            Case::UpperSnake => convert_case::Case::UpperSnake,
            Case::ScreamingSnake => convert_case::Case::Constant,
            Case::Kebab => convert_case::Case::Kebab,
            Case::Cobol => convert_case::Case::Cobol,
            Case::UpperKebab => convert_case::Case::UpperKebab,
            Case::Train => convert_case::Case::Train,
            Case::Flat => convert_case::Case::Flat,
            Case::UpperFlat => convert_case::Case::UpperFlat,
            Case::Alternating => convert_case::Case::Alternating,
            Case::Random => convert_case::Case::Random,
            Case::PseudoRandom => convert_case::Case::PseudoRandom,
        })
    }
}
