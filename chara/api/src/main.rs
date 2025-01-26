use std::sync::Arc;

use definitions::dto::definition::DefinitionDto;

use definitions::definitions::Definitions as DefinitionsImpl;
use engine::{
    definition::input::{BaseDefinitionInput, DefinedDefinitionInput},
    run, Definitions,
};
use rocket::serde::json::Json;

use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};

#[macro_use]
extern crate rocket;

#[post("/definitions", data = "<definition>")]
fn process_definition(definition: Json<DefinitionDto>) -> Json<DefinitionDto> {
    let definitions: Arc<dyn Definitions> = Arc::new(DefinitionsImpl {});

    let definition = run(definition.0.map(), definitions).unwrap();
    Json(DefinitionDto::from_definition(&definition))
}

#[get("/definitions/<id>")]
fn get_definition(id: &str) -> Json<DefinitionDto> {
    Json(DefinitionsImpl::read(&DefinedDefinitionInput::Id(id.to_string())).unwrap())
}

#[get("/definitions")]
fn list_definitions() -> Json<Vec<DefinitionDto>> {
    Json(DefinitionsImpl::all_definitions().unwrap())
}

#[launch]
fn rocket() -> _ {
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Patch]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true);

    rocket::build().attach(cors.to_cors().unwrap()).mount(
        "/api",
        routes![process_definition, get_definition, list_definitions],
    )
}
