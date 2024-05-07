use super::*;

#[test]
fn area_test() {
    let expected = 35;
    let actual = area(7, 5);
    assert_eq!(actual, expected);
}
