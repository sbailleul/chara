use std::{
    sync::Arc,
    thread::{self},
};

use contexts::ProcessorContext;
use definition::{Definition, DefinitionInput, ForeignDefinition};
use errors::DefinitionError;
use log::error;
use processor::ProcessorResult;
use types::{thread::Readonly, ThreadError};
pub mod cli;
pub mod contexts;
pub mod definition;
mod definition_test;
pub mod errors;
pub mod processor;
pub trait Definitions: Send + Sync {
    fn get(&self, definition: &DefinitionInput) -> Result<Definition, DefinitionError>;
    fn enrich(&self, context: &ProcessorContext) -> Result<ProcessorResult, DefinitionError>;
    fn save(&self, definition: &Definition) -> Result<(), DefinitionError>;
}

pub fn run(
    definition: Definition,
    definitions: Arc<dyn Definitions>,
) -> Result<Definition, DefinitionError> {
    let results = get_definitions(&definition, &definitions);
    for (foreign_definition, definition_output) in results {
        let mut foreign_definition = foreign_definition
            .write()
            .map_err(|_| DefinitionError::Thread(ThreadError::Poison))?;
        if let None = foreign_definition.output{
            foreign_definition.output = definition_output;
        }
    }
    let contexts = definition.processors_contexts();
    let results = enrich(contexts, definitions.clone());

    for (context, result) in results {
        let mut metadata = context
            .metadata
            .write()
            .map_err(|_| DefinitionError::Thread(ThreadError::Poison))?;
        if let Some(enrichment) = result.enrichment {
            if let (true, Some(mut edge_enrichment), Some(edge_context)) = (
                context.definition.write.edge,
                enrichment.edge,
                &context.definition.edge,
            ) {
                if let Some(edge) = metadata.edges.get_mut(&edge_context.name) {
                    edge.other.append(&mut edge_enrichment);
                }
            }
            if let (true, Some(mut metadata_enrichment)) =
                (context.definition.write.metadata, enrichment.metadata)
            {
                metadata.other.append(&mut metadata_enrichment);
            }
        }
        if let (Some(definition), Some(edge_context)) = (result.definition, context.definition.edge)
        {
            if let Some(edge) = metadata.edges.get_mut(&edge_context.name) {
                edge.definition = Some(definition);
            }
        }
    }
    definitions.save(&definition)?;
    // dbg!(&definition);
    Ok(definition)
}

fn get_definitions(
    definition: &Definition,
    definitions: &Arc<dyn Definitions>,
) -> Vec<(Readonly<ForeignDefinition>, Option<Definition>)> {
    definition
        .foreign_definitions
        .iter()
        .map(|definition| {
            let definition = definition.1.clone();
            let definitions = definitions.clone();
            thread::spawn(move || {
                definition
                    .read()
                    .map_err(|_| DefinitionError::Thread(ThreadError::Poison))
                    .and_then(|definition| definition.input.as_ref().map(|input| definitions.get(input)).transpose())
                    .map(|found_definition| (definition, found_definition))
                    
            })
        })
        .map(|handler| {
            handler
                .join()
                .map_err(|_err| DefinitionError::Thread(ThreadError::Join))
                .and_then(|res| res)
                .inspect_err(|err| error!("{err}"))
        })
        .flatten()
        .collect()
}

fn enrich(
    contexts: Vec<ProcessorContext>,
    definitions: Arc<dyn Definitions>,
) -> Vec<(ProcessorContext, ProcessorResult)> {
    contexts
        .into_iter()
        .map(|context| {
            let definitions = definitions.clone();
            thread::spawn(move || {
                definitions
                    .enrich(&context)
                    .map(|processor_result| (context, processor_result))
            })
        })
        .map(|handler| {
            handler
                .join()
                .map_err(|_err| DefinitionError::Thread(ThreadError::Join))
                .and_then(|res| res)
                .inspect_err(|err| error!("{err}"))
        })
        .flatten()
        .collect()
}
