use crate::args::CommandParse;
use crate::args::Commands;
use clap::Parser;
mod args;
mod retentionindex;
mod retentiontime;
mod retentiontimeadj;
use figlet_rs::FIGfont;
use retentionindex::retentionindexcal;
use retentiontime::retentionindex;
use retentiontimeadj::retentionadjust;

/*
Author Gaurav Sablok,
Email: codeprog@icloud.com
*/

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
        Commands::TimeRetention { filepath } => {
            let command = retentionindex(filepath).unwrap();
            println!("The retention time has been estimated{:?}", command);
        }
        Commands::RetentionTimeAdjust {
            filepath,
            retentionfactor,
        } => {
            let command = retentionadjust(filepath, retentionfactor).unwrap();
            println!("The value has been estimated:{:?}", command);
        }
    }
}
