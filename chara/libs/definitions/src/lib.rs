use std::{
    env,
    fs::{self, canonicalize, File},
    io::BufReader,
};

use cli::Cli;
use definition::{DefinitionDto, ProcessorResultDto};
use engine::{
    contexts::ProcessorContext,
    definition::{Definition, DefinitionInput},
    errors::CharaError,
    processor::{Enrichment, ProcessorResult},
    Definitions as ForeignDefinitions,
};
mod cli;
pub mod definition;
pub mod definitions;
mod mappers;
