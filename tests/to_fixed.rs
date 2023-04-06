// Translated from C to Rust. The original C code can be found at
// https://github.com/ulfjack/ryu and carries the following license:
//
// Copyright 2018 Ulf Adams
//
// The contents of this file may be used under the terms of the Apache License,
// Version 2.0.
//
//    (See accompanying file LICENSE-Apache or copy at
//     http://www.apache.org/licenses/LICENSE-2.0)
//
// Alternatively, the contents of this file may be used under the terms of
// the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE-Boost or copy at
//     https://www.boost.org/LICENSE_1_0.txt)
//
// Unless required by applicable law or agreed to in writing, this software
// is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.

#![allow(
    clippy::approx_constant,
    clippy::excessive_precision,
    clippy::cast_lossless,
    clippy::float_cmp,
    clippy::int_plus_one,
    clippy::non_ascii_literal,
    clippy::unreadable_literal,
    clippy::unseparated_literal_suffix
)]

#[macro_use]
mod macros;

use std::f64;

// FIXME: remove after testing
#[allow(unused_macros)]

fn pretty_to_fixed(f: f64, exp: u8) -> String {
    ryu_js::Buffer::new().format_to_fixed(f, exp).to_owned()
}

fn pretty_to_string(f: f64) -> String {
    ryu_js::Buffer::new().format(f).to_owned()
}

#[test]
#[should_panic]
fn range_over_100() {
    pretty_to_fixed(0.0, 101);
}

#[test]
fn nan() {
    for fraction_digits in 0..=100u8 {
        assert_eq!(pretty_to_fixed(f64::NAN, fraction_digits), "NaN");
    }
}

#[test]
fn infinity() {
    for fraction_digits in 0..=100u8 {
        assert_eq!(pretty_to_fixed(f64::INFINITY, fraction_digits), "Infinity");
    }
    for fraction_digits in 0..=100u8 {
        assert_eq!(
            pretty_to_fixed(f64::NEG_INFINITY, fraction_digits),
            "-Infinity"
        );
    }
}

#[test]
fn positive_zero() {
    assert_eq!(pretty_to_fixed(0.0, 0), "0");
    for fraction_digits in 1..=100u8 {
        let expected = "0".repeat(fraction_digits as usize);
        assert_eq!(
            pretty_to_fixed(0.0, fraction_digits),
            format!("0.{expected}")
        );
    }
}

#[test]
fn negative_zero() {
    assert_eq!(pretty_to_fixed(-0.0, 0), "0");
    for fraction_digits in 1..=100u8 {
        let expected = "0".repeat(fraction_digits as usize);
        assert_eq!(
            pretty_to_fixed(-0.0, fraction_digits),
            format!("0.{expected}")
        );
    }
}

// https://github.com/boa-dev/boa/issues/2609
#[test]
fn boa_issue_2609() {
    assert_eq!(pretty_to_fixed(1.25, 1), "1.3");
    assert_eq!(pretty_to_fixed(1.35, 1), "1.4");
}

#[test]
fn test262() {
    // test262 commit: be0abd93cd799a758714b5707fa87c9048fc38ce

    // test/built-ins/Number/prototype/toFixed/S15.7.4.5_A1.1_T02.js
    assert_eq!(pretty_to_fixed(1.0, 0), "1");
    assert_eq!(pretty_to_fixed(1.0, 1), "1.0");

    // test/built-ins/Number/prototype/toFixed/S15.7.4.5_A1.4_T01.js
    assert_eq!(pretty_to_fixed(1e21, 1), pretty_to_string(1e21));

    // test/built-ins/Number/prototype/toFixed/exactness.js
    assert_eq!(
        pretty_to_fixed(1000000000000000128.0, 0),
        "1000000000000000128"
    );
}

#[test]
fn test2() {
    assert_eq!(pretty_to_fixed(1010.95, 1), "1011.0");
}

#[test]
fn test3() {
    assert_eq!(pretty_to_fixed(1010.95, 0), "1011");
}

#[test]
fn test_to_fixed_100() {
    assert_eq!(pretty_to_fixed(1.0, 100), "1.0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(pretty_to_fixed(1.256, 100), "1.2560000000000000053290705182007513940334320068359375000000000000000000000000000000000000000000000000");
    assert_eq!(pretty_to_fixed(1.12345678910111213, 100), "1.1234567891011122409139488809159956872463226318359375000000000000000000000000000000000000000000000000");
}

#[test]
fn test_to_fixed_dont_round() {
    assert_eq!(pretty_to_fixed(1.25, 2), "1.25");
}
