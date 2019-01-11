extern crate test_adder;

#[test]
fn it_should_adds_two() {
    use test_adder::add_two;
    assert_eq!(4, add_two(2));
}