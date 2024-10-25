use std::{fs::File, io::BufReader};

use chara_dto::CharaDto;

use crate::{engine::chara::Chara, types::input::Definition};

mod chara_dto;
mod map;

pub fn parse(definition: &Definition) -> Chara {
    let result = match definition {
        Definition::File(path) => {
            let file = File::open(path).expect(format!("File {} isn't readable", path).as_str());
            let reader = BufReader::new(file);
            serde_json::from_reader(reader)
        }
        Definition::Json(content) => serde_json::from_str(&content),
    };
    let chara: CharaDto = result.expect(format!("Format {:?} isn't valid", definition).as_str());
    dbg!(&chara);
    chara.map()
}
