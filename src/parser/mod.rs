use std::{fs::File, io::BufReader};

use bootes_dto::BootesDto;

use crate::{engine::bootes::Bootes, types::input::Definition};

mod bootes_dto;

pub fn parse(definition: &Definition) -> Bootes {
    let result = match definition {
        Definition::File(path) => {
            let file = File::open(path).expect(format!("File {} isn't readable", path).as_str());
            let reader = BufReader::new(file);
            serde_json::from_reader(reader)
        }
        Definition::Json(content) => serde_json::from_str(&content),
    };
    let bootes: BootesDto = result.expect(format!("Format {:?} isn't valid", definition).as_str());
    dbg!(&bootes);
    bootes.map()
}
