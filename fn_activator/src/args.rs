use clap::clap_derive::Parser;

#[derive(Parser)]
pub struct Args {
    #[arg(short)]
    pub device: String,
}
