use clap::Parser;

#[derive(Parser, Default, Debug)]
#[clap(author, version, about)]
pub struct Args {
    pub url: String,
    #[clap(long)]
    pub widget_name: String,
    #[clap(long)]
    pub test: bool,
    #[clap(long)]
    pub args: Option<String>
}
