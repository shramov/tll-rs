use tll::stat::*;

#[repr(C)]
struct Example {
    rx: Field,
    tx: Field,
}

impl Default for Example {
    fn default() -> Example
    {
        Example {
            rx: Field::new("rx", Type::Sum),
            tx: Field::new_unit("tx", Type::Sum, Unit::Bytes)
        }
    }
}

/*
struct Borrow { data: i32 }
impl Borrow {
    pub fn acquire(&mut self) -> Option<&mut i32> { Some(&mut self.data) }
    pub fn release(&mut self, _i: &mut i32) {}
    //pub fn block(&mut self) -> Option<i32> { Some(20) }
}
*/

#[test]
fn test() {
    let mut data = Base::<Example>::new("test");
    {
        match data.acquire() {
            Some(mut r) => {
                let page = r.get();
                assert_eq!(page.rx.name(), "rx");
                assert_eq!(page.tx.name(), "tx");
                page.rx.update(10);
                page.tx.update(100);
            }
            None => assert!(false),
        }
    }

    {
        let b = data.block();
        assert_eq!(b.name(), "test");
        let p = b.acquire();
        assert!(p.is_some());
        assert!(b.acquire().is_none());
        b.release(&p.unwrap());
    }

    assert!(data.acquire().is_some());
}
