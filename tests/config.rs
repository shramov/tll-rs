use tll::config::*;

#[test]
fn test() {
    let mut cfg = Config::new();
    let mut v = cfg.get("a.b.c");
    assert!(v.is_none());
    cfg.set("a.b.c", "xxx");
    v = cfg.get("a.b.c");
    assert_eq!(v, Some ("xxx".to_string()));
    let sub = cfg.sub("a.b");
    assert!(!sub.is_none());
    let v1 = sub.unwrap().get("c");
    assert_eq!(v, v1);
}

#[test]
fn load() {
    let cfg = Config::load("yamls://{x: 1, a.b.c: 2}").unwrap();
    let v = cfg.get("a.b.c");
    assert!(v == Some ("2".to_string()));
}

#[test]
fn browse() {
    let mut cfg = Config::new();
    let mut v = cfg.get("a.b.c");
    assert!(v.is_none());
    cfg.set("a.b.c", "xxx");
    v = cfg.get("a.b.c");
    assert!(v == Some ("xxx".to_string()));
}

#[test]
fn test_type() {
    let cfg = Config::load("yamls://
i8: -100
u8: 200
f64: 1.234
bool: true
").unwrap();
    assert_eq!(-100, cfg.get_typed::<i8>("i8", 0).unwrap());
    assert_eq!(200, cfg.get_typed::<u8>("u8", 0).unwrap());
    assert_eq!(1.234, cfg.get_typed::<f64>("f64", 0.).unwrap());
    assert_eq!(true, cfg.get_typed::<bool>("bool", false).unwrap());
}

