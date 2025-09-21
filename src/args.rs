use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(
    name = "retentiontools for mass spectrometry",
    version = "1.0",
    about = "rustRET.
           ************************************************
          Gaurav Sablok, IBCH, PAN, Poznan, Poland,
          https://portal.ichb.pl/laboratory-of-genomics/.
          Email: gsablok@ibch.poznan.pl
          ************************************************"
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
        /// provide the path to the retetnion file
        csvfile: String,
    },
    /// retention time calculate
    TimeRetention {
        /// provide the path to the file for the retention time calculation
        filepath: String,
    },
    /// retention time adjusted calculation
    RetentionTimeAdjust {
        /// provide the path to the file,
        filepath: String,
        retentionfactor: String,
    },
}
