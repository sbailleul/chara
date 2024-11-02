use clap::{command, Parser};
use definitions::DefinitionContextDto;
use thiserror::Error;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    context: String,
}
#[derive(Debug, Error)]
enum Error {
    #[error("Parse error {0}")]
    Parse(serde_json::Error),
}
struct Metadata{
        
}
#[tokio::main]
async fn main() -> Result<(), Error> {
    colog::init();
    let args = Args::parse();
    serde_json::from_str::<DefinitionContextDto>(&args.context)
        .map_err(Error::Parse)
        .map(|context| {
            octocrab::instance().workflows("sbailleul", repo);

        });
    Ok(())
}
