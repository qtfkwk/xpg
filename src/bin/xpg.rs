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

    /// Print default configuration
    #[arg(short, long)]
    default_config: bool,

    /// Print readme
    #[arg(short, long)]
    readme: bool,

    /// Pattern (see note 1 below)
    #[arg(default_value = "wwww")]
    pattern: String,
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

    if cli.default_config {
        print!("{}", xpg::CONFIG.dump()?);
        std::process::exit(0);
    }

    let cfg = if let Some(config) = cli.config {
        // Load a custom configuration via -C
        xpg::Config::from_path(&config)?
    } else if let Some(proj_dirs) = directories::ProjectDirs::from("com", "qtfkwk", "xpg") {
        let config_dir = proj_dirs.config_dir();
        let user_config = config_dir.join("config.json");
        if user_config.exists() {
            // Load a custom configuration file
            xpg::Config::from_path(&user_config)?
        } else {
            // Use the default configuration
            xpg::CONFIG.clone()
        }
    } else {
        // Use the default configuration
        xpg::CONFIG.clone()
    };

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

    // Pad pattern to the minimum length with last character of pattern if it doesn't have any words
    let min = length.start();
    let len = cli.pattern.len();
    let d = if len < *min { min - len } else { 0 };
    let pattern = if d > 0
        && !["keychain", "codename", "codename-series"].contains(&cli.pattern.as_str())
        && !cli.pattern.contains(['w', 'W', 'T'])
        && !xpg::WORD_KIND_SUBS
            .iter()
            .any(|(x, _)| cli.pattern.contains(x))
    {
        format!(
            "{}{}",
            cli.pattern,
            format!("{}", cli.pattern.chars().last().unwrap()).repeat(d),
        )
    } else {
        cli.pattern.clone()
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
                _ => cfg.generate(&pattern),
            };

            // Enforce length requirements
            if length.contains(&pw.len()) {
                println!("{pw}");
                break;
            } else {
                attempts += 1;
                if attempts >= cli.attempts {
                    exit(1, "Maximum attempts exceeded!");
                }
            }
        }
        i += 1;
        if count > 0 && i >= count {
            break;
        }
    }

    Ok(())
}

fn exit(code: i32, msg: &str) {
    eprintln!("ERROR: {msg}");
    std::process::exit(code);
}
