extern crate hyper;
extern crate json;

use std::io::{Write, Read};
use std::path::Path;
use std::fs::File;
use std::env;


fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let mut w3_resp =
        hyper::Client::new().get("https://www.w3.org/TR/html5/entities.json").send().unwrap();

    let mut body = String::new();
    w3_resp.read_to_string(&mut body).unwrap();

    let entities = json::parse(&body).unwrap();
    let mut out_file = File::create(&Path::new(&out_dir).join("entity_list.rs")).unwrap();

    writeln!(out_file,
             "pub static ENTITIES: [Entity; {}] = [",
             entities.len())
        .unwrap();
    for (ent, dat) in entities.entries() {
        let cp_tp = match dat["codepoints"].len() {
            1 => "Single",
            2 => "Double",
            l => panic!("Unsupported {}-length codepoint", l),
        };
        let mut chars = dat["characters"].dump();
        chars.pop();
        chars.remove(0);

        writeln!(out_file,
            r#"    Entity {{ entity: "{}", codepoints: Codepoints::{}({}), characters: "{}" }},"#,
            ent, cp_tp, dat["codepoints"], chars)
            .unwrap();
    }
    writeln!(out_file, "];").unwrap();
}
