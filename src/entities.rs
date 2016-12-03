pub enum Codepoints {
    Single([u32; 1]),
    Double([u32; 2]),
}


pub struct Entity {
    pub entity: &'static str,
    pub codepoints: Codepoints,
    pub characters: &'static str,
}


include!(concat!(env!("OUT_DIR"), "/entity_list.rs"));
