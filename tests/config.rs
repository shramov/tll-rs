use tll::config::*;

#[test]
fn test() {
    let mut cfg = Config::new();
    let mut v = cfg.get("a.b.c");
    assert!(v.is_none());
    cfg.set("a.b.c", "xxx");
    v = cfg.get("a.b.c");
    assert_eq!(v, Some("xxx".to_string()));
    let sub = cfg.sub("a.b");
    assert!(!sub.is_none());
    let v1 = sub.unwrap().get("c");
    assert_eq!(v, v1);
}

#[test]
fn load() {
    let cfg = Config::load("yamls://{x: 1, a.b.c: 2}").unwrap();
    let v = cfg.get("a.b.c");
    assert!(v == Some("2".to_string()));
}

#[test]
fn browse() {
    let mut cfg = Config::new();
    assert!(cfg.get("a.b.0").is_none());
    cfg.set("a.b.0", "xxx");
    cfg.set("a.b.1", "yyy");
    cfg.set("a.b.2", "zzz");
    assert!(cfg.get("a.b.0") == Some("xxx".to_string()));
    assert_eq!(
        cfg.browse("a.b.**").iter().map(|x| (x.0.clone(), x.1.get_self())).collect::<Vec<_>>(),
        vec![("a.b.0", "xxx"), ("a.b.1", "yyy"), ("a.b.2", "zzz")]
            .iter()
            .map(|&x| (x.0.to_owned(), Some(x.1.to_owned())))
            .collect::<Vec<_>>()
    );
}

#[test]
fn test_type() {
    let cfg = Config::load(
        "yamls://
i8: -100
u8: 200
f64: 1.234
bool: true
",
    )
    .unwrap();
    assert_eq!(-100, cfg.get_typed::<i8>("i8", 0).unwrap());
    assert_eq!(200, cfg.get_typed::<u8>("u8", 0).unwrap());
    assert_eq!(1.234, cfg.get_typed::<f64>("f64", 0.).unwrap());
    assert_eq!(true, cfg.get_typed::<bool>("bool", false).unwrap());
}

#[test]
fn test_chain() {
    let mut c0 = Config::new();
    c0.set("a", "10");
    c0.set("b", "10");
    c0.set("d", "");
    let mut c1 = Config::new();
    c1.set("a", "20");
    c1.set("c", "20");

    let chain = ConfigChain::new(Some(c0), None, Some(c1));

    assert_eq!(chain.get("a"), Some("10".into()));
    assert_eq!(chain.get("b"), Some("10".into()));
    assert_eq!(chain.get("c"), Some("20".into()));
    assert_eq!(chain.get("d"), Some("".into()));
    assert_eq!(chain.get("e"), None);
}
