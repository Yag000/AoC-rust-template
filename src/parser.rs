use clap_derive::Parser;

#[derive(Parser)]
pub struct CommandArgument {
    pub day: Option<u32>,
    pub part: Option<u32>,
    #[clap(short, long)]
    pub publish: bool,
}