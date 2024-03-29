extern crate entities;
extern crate serde_json;
use entities::ENTITIES;
use serde_json::{json, Value};
use std::fs::File;
use std::io::BufReader;

fn load_entities_spec() -> Value {
    let file = File::open("./downloaded-entities.json")
        .or_else(|_| File::open("./tests/entities.json"))
        .expect("failed to open ./downloaded-entities or ./tests/entities.json");
    let reader = BufReader::new(file);

    serde_json::from_reader(reader).unwrap()
}

#[test]
fn check_spec_conformity() {
    let spec = load_entities_spec();

    for e in ENTITIES.iter() {
        assert_eq!(
            spec[e.entity]["characters"],
            json!(e.characters),
            "failed while comparing characters for entity {}",
            e.entity
        );

        match e.codepoints {
            entities::Codepoints::Single(x) => {
                assert_eq!(
                    spec[e.entity]["codepoints"],
                    json!([x]),
                    "failed while comparing codepoints for entity {}",
                    e.entity
                )
            }
            entities::Codepoints::Double(x, y) => {
                assert_eq!(
                    spec[e.entity]["codepoints"],
                    json!([x, y]),
                    "failed while comparing codepoints for entity {}",
                    e.entity
                )
            }
        }
    }

    for entity in spec
        .as_object()
        .expect("spec file doesn't contain a JSON object")
        .keys()
    {
        let found = ENTITIES.iter().find(|e| e.entity == entity);
        assert!(
            found.is_some(),
            "entity {} is in spec but not in crate",
            entity
        );
    }

    assert_eq!(
        spec.as_object().unwrap().len(),
        ENTITIES.iter().count(),
        "spec and crate have different number of entities"
    );
}
