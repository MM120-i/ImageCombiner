use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Args {
    #[arg(long, help = "Path to first input image")]
    pub image_1: String,

    #[arg(long, help = "Path to second input image")]
    pub image_2: String,

    #[arg(short, long, help = "Path to output image")]
    pub output: String,
}

impl Args {
    pub fn new() -> Self {
        Args::parse()
    }
}