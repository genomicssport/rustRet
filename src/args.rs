use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(
    name = "retentiontools for mass spectrometry",
    version = "1.0",
    about = "rustRet"
)]
pub struct CommandParse {
    /// subcommands for the specific actions
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// retention index calculation
    RetentionIndex {
        /// provide the path to the alignment file
        csvfile: String,
    },
}
