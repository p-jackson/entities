#![no_std]


mod entities;


pub use entities::ENTITIES;


#[cfg(test)]
mod tests {
    extern crate std;
    use self::std::collections::HashMap;
    use super::ENTITIES;

    #[test]
    fn sanity() {
        let found = ENTITIES.iter().find(|e| e.entity == "&gt;");
        assert!(found.is_some());
        assert_eq!(found.unwrap().characters, ">");
    }

    #[test]
    fn make_map() {
        let mut mapping = HashMap::<&'static str, &'static str>::new();
        mapping.insert(ENTITIES[0].entity, ENTITIES[0].characters);
        assert_eq!(mapping.get(ENTITIES[0].entity), Some(&ENTITIES[0].characters));
    }
}
