use adder;

mod common;
#[test]
fn it_adds_two() {
    common::example();
    assert_eq!(4, adder::add_two(2));
}