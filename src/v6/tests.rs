use std::mem::drop;

#[test]
fn equality() {
    let a = super::Allocator::<i64>::new(1);
    let b = a.box_it(123);
    assert_eq!(123, *b);
}

#[test]
#[should_panic]
fn panic_when_out_of_memory() {
    let a = super::Allocator::<i64>::new(1);
    let b = a.box_it(123);
    let c = a.box_it(234);
    drop((b, c));
}

#[test]
fn memory_reclamation() {
    let a = super::Allocator::<i64>::new(1);
    let b = a.box_it(123);
    drop(b);
    let c = a.box_it(234);
    assert_eq!(234, *c);
}
