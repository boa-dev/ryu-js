#![no_main]

use libfuzzer_sys::fuzz_target;
use std::mem;

macro_rules! ryu_js_test {
    ($val:expr, $method:ident) => {
        match $val {
            val => {
                let mut buffer = ryu_js::Buffer::new();
                let string = buffer.$method(val);
                assert!(string.len() <= mem::size_of::<ryu_js::Buffer>());
                if val.is_finite() {
                    assert_eq!(val, string.parse().unwrap());
                }
            }
        }
    };
}

fuzz_target!(|inputs: (f64, bool)| {
    let (input, finite) = inputs;
    match (input, finite) {
        (val, false) => ryu_js_test!(val, format),
        (val, true) => ryu_js_test!(val, format_finite),
    }
});
