use tll::config::*;

#[test]
fn test() {
    let cfg = Config::new();
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
    let cfg = Config::new();
    let mut v = cfg.get("a.b.c");
    assert!(v.is_none());
    cfg.set("a.b.c", "xxx");
    v = cfg.get("a.b.c");
    assert!(v == Some ("xxx".to_string()));
}
