use rs16_testing_in_rust::*;
mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, add_2(2));
}