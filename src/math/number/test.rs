use super::*;

#[test]
fn test_rational() {
    assert_eq!(Number::rational(6, 9), Number::Rat(2, 3));
    assert_eq!(Number::rational(-6, 9), Number::Rat(-2, 3));
    assert_eq!(Number::rational(6, -9), Number::Rat(-2, 3));
    assert_eq!(Number::rational(-6, -9), Number::Rat(2, 3));
    assert_eq!(Number::rational(6, 2), Number::Int(3));
    assert_eq!(Number::rational(-6, 2), Number::Int(-3));
}

#[test]
fn test_ops() {
    assert_eq!(
        Number::rational(1, 3) + Number::rational(3, 4),
        Number::rational(13, 12)
    );
    assert_eq!(
        Number::rational(1, 3) * Number::rational(3, 4),
        Number::rational(1, 4)
    );
}
