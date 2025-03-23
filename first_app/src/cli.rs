use std::path::PathBuf;
use clap::Parser;

// Define arguments of the CLI app
#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Args {
    /// Path of the input JSON file
    #[clap(short, long)]
    input: PathBuf,

    /// Path of the output target (Output on standard output if the output path isn't set)
    #[clap(short, long)]
    output: Option<PathBuf>,
}

impl Args {
    pub fn input(&self) -> &PathBuf {
        &self.input
    }
    
    pub fn output(&self) -> Option<&PathBuf> {
        self.output.as_ref()
    }
}