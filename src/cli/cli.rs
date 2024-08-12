use clap::Parser;
use std::str::FromStr;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub wicked: String,
    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    pub count: u8,
}
#[derive(Debug, PartialEq)]
enum Command {
    GenerateKeyPair,
    SignMessage(String),
}
impl FromStr for Command {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "generate_kp" => Ok(Command::GenerateKeyPair),
            "sign" => Ok(Command::SignMessage("test".into())),
            _ => Err("Doesn't match".to_owned()),
        }
    }
}
