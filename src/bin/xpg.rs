use clap::Parser;

#[derive(Parser)]
#[command(about, version, max_term_width = 80)]
struct Cli {
    /// Password pattern
    /// (`W`: WORD, `w`: word, `T`: Word, `k`: `shuffle(cccc(c|d)(C|c))`
    /// `C`: A-Z, `c`: a-z, `d`: 0-9, `s`: `~!@#$%^&*-_=+;:,./?()[]{}<>`, `a`: `C|c|d|s`)
    #[arg(short, long, default_value = "wwww")]
    pattern: String,

    /// Generate keychain-style password(s); equivalent to `-p 'k-k-k'`
    #[arg(long, conflicts_with_all = ["code_name", "pattern", "length", "minimum", "maximum"])]
    keychain: bool,

    /// Generate code name(s); equivalent to `-p 'W W'`
    #[arg(long, conflicts_with = "pattern")]
    code_name: bool,

    /// Number of passwords
    #[arg(short, long, value_name = "NUMBER", default_value = "1")]
    count: usize,

    /// Length
    #[arg(long, value_name = "NUMBER", conflicts_with_all = ["minimum", "maximum"])]
    length: Option<usize>,

    /// Minimum length
    #[arg(long, value_name = "NUMBER")]
    minimum: Option<usize>,

    /// Maximum length
    #[arg(long, value_name = "NUMBER")]
    maximum: Option<usize>,

    /// Attempts
    #[arg(long, value_name = "NUMBER", default_value = "10")]
    attempts: usize,
}

/**

Provides the command line interface

*/
fn main() {
    let cli = Cli::parse();

    let pattern = if cli.keychain {
        String::from("k-k-k")
    } else if cli.code_name {
        String::from("W W")
    } else {
        cli.pattern.clone()
    };

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

    // Generate password(s)
    for _ in 0..cli.count {
        let mut attempts = 0;
        loop {
            let pw = if cli.keychain {
                xpg::keychain()
            } else if cli.code_name {
                xpg::code_name()
            } else {
                xpg::generate(&pattern)
            };
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
    }
}

fn exit(code: i32, msg: &str) {
    eprintln!("ERROR: {msg}");
    std::process::exit(code);
}
