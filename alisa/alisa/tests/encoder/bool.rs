
use crate::encode;

#[test]
fn bool() {

    assert_eq!(encode(|enc| {
        enc.boolean(false)
    }), &[0b11000000]);

    assert_eq!(encode(|enc| {
        enc.boolean(true)
    }), &[0b11000001]);

}
