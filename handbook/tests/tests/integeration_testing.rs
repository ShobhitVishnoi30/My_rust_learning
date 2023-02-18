mod common;
use tests;

#[test]
fn it_adds_two() {
    assert_eq!(5, tests::add_two(2, 3));
}
