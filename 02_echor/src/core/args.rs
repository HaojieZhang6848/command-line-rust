use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `echo`
pub struct Args {
    /// Input text
    #[arg(required(true))]
    pub text: Vec<String>,

    /// Do not print newline
    #[arg(short('n'))]
    pub omit_newline: bool,
}
