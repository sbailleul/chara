use std::fs;

use clap::{command, Parser};
use context::DefinitionContext;
use definitions::definition::DefinitionContextDto;
use dtos::WorkflowDto;
use errors::Error;
use github::GithubContext;
use log::{error, info};
mod context;
mod dtos;
mod errors;
mod github;
mod workflow;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    context: String,
    #[arg(short, long)]
    output: String,
    #[arg(short, long)]
    server: Option<String>,
    #[arg(short, long)]
    app_id: Option<u64>,
    #[arg(short, long)]
    installation_id: Option<u64>,
    #[arg(short, long)]
    private_key: Option<String>,
}
/*
{\"location\":\"/home/sbailleul/code/chara/examples/chara.json\",\"write\":{\"metadata\":false,\"edge\":true},\"metadata\":{\"name\":\"build\",\"value\":{\"file\":\".github/workflows/build-workflow.yaml\",\"owner\":\"sbailleul\",\"repository\":\"chara_private\"}},\"edge\":{\"name\":\"#/workflows\",\"value\":{}}}
*/
#[tokio::main]
async fn main() -> Result<(), Error> {
    colog::init();
    let args = Args::parse();
    let res = match serde_json::from_str::<DefinitionContextDto>(&args.context)
        .map_err(Error::Json)
        .and_then(|ctx: DefinitionContextDto| {
            DefinitionContext::new(
                ctx,
                GithubContext::new(
                    args.server,
                    args.app_id,
                    args.private_key,
                    args.installation_id,
                )?,
            )
        }) {
        Ok(context) => {
            let content = context.workflow_content().await?;
            info!("Read {content}");
            let res: WorkflowDto = serde_yaml::from_str(&content).map_err(Error::Yaml)?;
            let res = res.into();
            if let Ok(def) = res
                .to_processor_result(context)
                .and_then(|def| serde_json::to_string(&def).map_err(Error::Json))
            {
                fs::write(args.output, &def).map_err(Error::IO)?;
            }
            Ok(())
        }
        Err(err) => Err(err),
    };
    if let Err(err) = &res {
        error!("{err}");
    }
    res
}
