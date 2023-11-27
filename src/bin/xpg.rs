use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(
    about,
    version,
    max_term_width = 80,
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
        * `haiku`
        * `haiku+`
        * `haiku-`
        * `haiku--`
        * `haiku++`
        * `haiku+++`

2. By default, extended words are only available in exclusively extended word
   kinds; however, with `-e`, they are merged with regular word kinds; use with
   the `-L` option to see its effect.
"
)]
struct Cli {
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

    /// Attempts
    #[arg(short, long, value_name = "NUMBER", default_value = "10")]
    attempts: usize,

    /// Configuration file
    #[arg(short = 'C', long, value_name = "PATH")]
    config: Option<PathBuf>,

    /// Merge extended words (see note 2)
    #[arg(short, long)]
    extended: bool,

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

/**

Provides the command line interface

*/
fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.readme {
        print!("{}", include_str!("../../README.md"));
        std::process::exit(0);
    }

    // Process the configuration
    let cfg = if let Some(config) = cli.config {
        // Load a custom configuration via -C
        xpg::Config::from_path(&config, cli.extended)?
    } else if let Some(proj_dirs) = directories::ProjectDirs::from("com", "qtfkwk", "xpg") {
        let config_dir = proj_dirs.config_dir();
        let user_config = config_dir.join("config.json");
        if user_config.exists() {
            // Load a custom configuration file
            xpg::Config::from_path(&user_config, cli.extended)?
        } else {
            // Load the default configuration
            xpg::Config::from_str(xpg::CONFIG, cli.extended)?
        }
    } else {
        // Load the default configuration
        xpg::Config::from_str(xpg::CONFIG, cli.extended)?
    };

    if cli.dump_config {
        print!("{}", cfg.dump()?);
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
                    "haiku+" => cfg.haiku(xpg::HaikuVariant::WithSyllables),
                    "haiku-" => cfg.haiku(xpg::HaikuVariant::WithSyllablesCondensed),
                    "haiku--" => cfg.haiku(xpg::HaikuVariant::Condensed),
                    "haiku++" => cfg.haiku(xpg::HaikuVariant::Full),
                    "haiku+++" => cfg.haiku(xpg::HaikuVariant::FullWithSyllables),
                    _ => cfg.generate(&pattern),
                };

                // Enforce length requirements
                if length.contains(&pw.len()) {
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
