use std::{
    sync::Arc,
    thread::{self},
};

use common::{
    merge::Merge,
    thread::{readonly, Readonly},
    ThreadError,
};
use contexts::ProcessorContext;

use definition::{foreign_definition::ForeignDefinition, input::DefinedDefinitionInput};
use draft::draft_definition::{ DraftDefinition};
use errors::CharaError;
use log::error;
use processor::ProcessorResult;
pub mod clean;
pub mod cli;
pub mod contexts;
pub mod definition;
mod definition_test;
pub mod draft;
pub mod errors;
pub mod processor;
pub mod reference_value;
pub trait Definitions: Send + Sync {
    fn get(&self, definition: &DefinedDefinitionInput) -> Result<DraftDefinition, CharaError>;
    fn enrich(&self, context: &ProcessorContext) -> Result<ProcessorResult, CharaError>;
    fn save(&self, definition: &DraftDefinition) -> Result<(), CharaError>;
}

pub fn run(
    definition: DraftDefinition,
    definitions: Arc<dyn Definitions>,
) -> Result<DraftDefinition, CharaError> {
    let definition = process_definition(readonly(definition.clone()), &definitions)?;
    definitions.save(&definition)?;
    Ok(definition)
}

fn process_definition(
    definition: Readonly<DraftDefinition>,
    definitions: &Arc<dyn Definitions>,
) -> Result<DraftDefinition, CharaError> {
    let definition_value = definition
        .read()
        .map_err(|_| CharaError::Thread(ThreadError::Poison))?;
    let results = get_definitions(&definition_value, definitions);
    for (foreign_definition, definition_output) in results {
        let mut foreign_definition = foreign_definition
            .write()
            .map_err(|_| CharaError::Thread(ThreadError::Poison))?;
        if let None = foreign_definition.output {
            foreign_definition.output.merge(&definition_output);
        }
    }
    let contexts = definition_value.processors_contexts();
    let results = enrich(contexts, definitions.clone());

    handle_results(definition.clone(), results, definitions)?;
    Ok(definition_value.to_owned())
}

fn handle_results(
    source_definition: Readonly<DraftDefinition>,
    results: Vec<(ProcessorContext, ProcessorResult)>,
    definitions: &Arc<dyn Definitions>,
) -> Result<(), CharaError> {
    for (context, result) in results {
        let mut metadata = context
            .metadata
            .write()
            .map_err(|_| CharaError::Thread(ThreadError::Poison))?;
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
        if let (Some(result_definition), Some(edge_context)) =
            (result.definition, context.definition.edge)
        {
            if let Some(edge) = metadata.edges.get_mut(&edge_context.name) {
                // if let Ok(src_edge) = edge.edge.read() {
                //     if let Some(foreign_definition) = src_edge.definition.as_ref() {
                //         if let Ok(foreign_definition) = foreign_definition.read() {
                //             if let Some(foreign_definition) = foreign_definition.output.as_ref() {
                //                 result_definition.merge(foreign_definition);
                //             }
                //         }
                //     }
                // }
                // edge.definition.merge(&Some(result_definition));
                if let Some(definition) = edge.definition.as_mut() {
                    definition.parent = Some(source_definition.clone());
                    *definition = process_definition(readonly(definition.clone()), definitions)?;
                }
            }
        }
    }
    Ok(())
}

fn get_definitions(
    definition: &DraftDefinition,
    definitions: &Arc<dyn Definitions>,
) -> Vec<(Readonly<ForeignDefinition>, Option<DraftDefinition>)> {
    definition
        .foreign_definitions
        .iter()
        .map(|definition| {
            let definition = definition.1.clone();
            let definitions: Arc<dyn Definitions> = definitions.clone();
            thread::spawn(move || {
                definition
                    .read()
                    .map_err(|_| CharaError::Thread(ThreadError::Poison))
                    .and_then(|definition| {
                        definition
                            .input
                            .as_ref()
                            .and_then(|input| input.to_defined())
                            .map(|input| definitions.get(&input))
                            .transpose()
                    })
                    .map(|found_definition| (definition, found_definition))
            })
        })
        .map(|handler| {
            handler
                .join()
                .map_err(|_err| CharaError::Thread(ThreadError::Join))
                .and_then(|res| res)
                .inspect_err(|err| error!("get_definitions {err}"))
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
                .map_err(|_err| CharaError::Thread(ThreadError::Join))
                .and_then(|res| res)
                .inspect_err(|err| error!("enrich {err}"))
        })
        .flatten()
        .collect()
}
