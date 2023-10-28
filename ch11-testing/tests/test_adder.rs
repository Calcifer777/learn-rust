use ch11_testing::*;

#[test]
fn test_adder() {
    let out = ch11_testing::adder(2);
    assert!(out==5, "adder(2) = {} != 5", out);
}

#[test]
fn test_with_assert_eq() {
    let out = adder(2);
    assert_eq!(out, 5);
}

#[test]
fn test_handled_failure() -> Result<(), String> {
    let exp = 42;
    let out = adder(1);
    if out == exp {
        Ok(())
    } else {
        Err(format!("Expected {} but got {}", exp, out))
    }
}

#[test]
#[should_panic]
fn test_expected_failure() {
    panic!("Make this test fail");
}