use anyhow::Result;
use clap::Parser;
use xpg::*;

#[derive(Parser)]
#[command(about, version, max_term_width = 80)]
struct Cli {
    /// Calculate total number of possible passwords for the specified number of words
    #[arg(long)]
    analyze: bool,

    /// Number of words in password
    #[arg(short, long, value_name = "NUMBER", default_value = "4")]
    words: usize,

    /// Number of passwords
    #[arg(short, long, value_name = "NUMBER", default_value = "1")]
    count: usize,

    /// Append digits
    #[arg(short, long, value_name = "NUMBER", default_value = "0")]
    digits: usize,

    /// Append symbols
    #[arg(short, long, value_name = "NUMBER", default_value = "0")]
    symbols: usize,

    /// Append lowercase letters
    #[arg(short, long, value_name = "NUMBER", default_value = "0")]
    lowercase: usize,

    /// Append uppercase letters
    #[arg(short, long, value_name = "NUMBER", default_value = "0")]
    uppercase: usize,

    /// Append *any* characters (digits, symbols, lowercase, uppercase)
    #[arg(short, long, value_name = "NUMBER", default_value = "0")]
    any: usize,

    /// Generate keychain-style password(s)
    #[arg(long, conflicts_with_all = ["code_name", "analyze"])]
    keychain: bool,

    /// Generate code name(s)
    #[arg(long, conflicts_with_all = ["keychain", "analyze"])]
    code_name: bool,
}

/**

Provides the command line interface

*/
fn main() -> Result<()> {
    let a = Cli::parse();

    if a.words == 0 && a.any == 0 {
        exit(101, "Either --words or --any must be greater than zero!");
    }

    if a.analyze {
        print!("{}", analyze(a.words)?);
        return Ok(());
    }

    // Generate password(s)
    for _ in 0..a.count {
        println!(
            "{}",
            if a.keychain {
                keychain()
            } else if a.code_name {
                code_name()
            } else if a.digits + a.symbols + a.lowercase + a.uppercase + a.any > 0 {
                xpg_plus(
                    a.words,
                    a.digits,
                    a.symbols,
                    a.lowercase,
                    a.uppercase,
                    a.any,
                )
            } else {
                xpg!(a.words)
            },
        );
    }

    Ok(())
}

fn exit(code: i32, msg: &str) {
    eprintln!("ERROR: {msg}");
    std::process::exit(code);
}
