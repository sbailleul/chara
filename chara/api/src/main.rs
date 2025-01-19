use std::sync::Arc;

use definitions::definition::DefinitionDto;

use definitions::definitions::Definitions as DefinitionsImpl;
use engine::{definition::input::BaseDefinitionInput, run, Definitions};
use rocket::serde::json::Json;
#[macro_use]
extern crate rocket;

#[post("/definitions", data = "<definition>")]
fn index(definition: Json<DefinitionDto>) -> Json<DefinitionDto> {
    let definitions: Arc<dyn Definitions> = Arc::new(DefinitionsImpl {});

    let definition = run(definition.0.map(), definitions).unwrap();
    Json(DefinitionDto::from_definition(&definition))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
