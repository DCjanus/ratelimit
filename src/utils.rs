/// saturating integer cast
pub trait SaturatingCast<To>: Sized {
    fn saturating_cast(self) -> To;
}

impl SaturatingCast<u64> for u128 {
    fn saturating_cast(self) -> u64 {
        if self >= u64::max_value() as u128 {
            u64::max_value()
        } else {
            self as u64
        }
    }
}

#[test]
fn test_saturating_cast() {
    assert_eq!(
        (u64::max_value() as u128 + 1).saturating_cast(),
        u64::max_value()
    );
    assert_eq!(32.saturating_cast(), 32u64);
}
