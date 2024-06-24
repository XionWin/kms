use pencil_rs::def::{Command, Point};


#[allow(dead_code)]
pub fn pencil_test() {
    let mut cache = pencil_rs::cache::PathCache::default();
    println!("cache: {:#?}", cache);
    let commands = vec![
        Command::MoveTo(Point::new(0.0, 0.0)),
        Command::LineTo(Point::new(100.0, 0.0)),
        Command::LineTo(Point::new(100.0, 100.0)),
        Command::Solidity(pencil_rs::def::Solidity::Solid),
        Command::Close
    ];
    cache.flatten_paths(&commands, 0.1, 0.1);

}