use tll::logger::*;

#[test]
fn test() {
    let l = Logger::new("test");
    l.info(&format!("Test {}", l.name()))
}
