use super::*;

#[test]
fn test_rational() {
    assert_eq!(Num::rational(6, 9), Num::Rat(2, 3));
    assert_eq!(Num::rational(-6, 9), Num::Rat(-2, 3));
    assert_eq!(Num::rational(6, -9), Num::Rat(-2, 3));
    assert_eq!(Num::rational(-6, -9), Num::Rat(2, 3));
    assert_eq!(Num::rational(6, 2), Num::Int(3));
    assert_eq!(Num::rational(-6, 2), Num::Int(-3));
}

#[test]
fn test_ops() {
    assert_eq!(
        Num::rational(1, 3) + Num::rational(3, 4),
        Num::rational(13, 12)
    );
    assert_eq!(
        Num::rational(1, 3) * Num::rational(3, 4),
        Num::rational(1, 4)
    );
}
