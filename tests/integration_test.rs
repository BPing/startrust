mod common;

use startrust::test_demo;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, test_demo::add_func(2, 2));
}