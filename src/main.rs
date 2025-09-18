use crate::args::CommandParse;
use crate::args::Commands;
use clap::Parser;
mod args;
mod retentionindex;
use figlet_rs::FIGfont;
use retentionindex::retentionindexcal;

/*
* Author Gaurav Sablok
*
* a complete set of the mass spectrometry tools for the retention time and retention index calculation.
*
* */

fn main() {
    let fontgenerate = FIGfont::standard().unwrap();
    let repgenerate = fontgenerate.convert("rustRET");
    println!("{}", repgenerate.unwrap());
    let argsparse = CommandParse::parse();
    match &argsparse.command {
        Commands::RetentionIndex { csvfile } => {
            let command = retentionindexcal(csvfile).unwrap();
            println!("The merged alignment have been written: {:?}", command);
        }
    }
}
