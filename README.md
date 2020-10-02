# Ryū-js

Ryū-js is a fork of the [ryu][ryu-crate] crate adjusted to comply to the ECMAScript [number-to-string][number-to-string] algorithm.
This crate is used in the [boa][boa-crate] crate for number to string conversions.

[ryu-crate]: https://crates.io/crates/ryu
[boa-crate]: https://crates.io/crates/Boa
[number-to-string]: https://tc39.es/ecma262/#sec-numeric-types-number-tostring

Pure Rust implementation of Ryū, an algorithm to quickly convert floating point
numbers to decimal strings.

The PLDI'18 paper [*Ryū: fast float-to-string conversion*][paper] by Ulf Adams
includes a complete correctness proof of the algorithm. The paper is available
under the creative commons CC-BY-SA license.

This Rust implementation is a line-by-line port of Ulf Adams' implementation in
C, [https://github.com/ulfjack/ryu][upstream].

*Requirements: this crate supports any compiler version back to rustc 1.31; it
uses nothing from the Rust standard library so is usable from no_std crates.*

[paper]: https://dl.acm.org/citation.cfm?id=3192369
[upstream]: https://github.com/ulfjack/ryu/tree/b38c7cc223939a57c58a4e7a68e8cc87f2e6689f

```toml
[dependencies]
ryu-js = "0.2"
```

## Example

```rust
fn main() {
    let mut buffer = ryu_js::Buffer::new();
    let printed = buffer.format(1.234);
    assert_eq!(printed, "1.234");
}
```

## Performance

The benchmarks measure the average time to print a 32-bit float and average
time to print a 64-bit float, where the inputs are distributed as uniform random
bit patterns 32 and 64 bits wide.

The upstream C code, the unsafe direct Rust port, and the safe pretty Rust API
all perform the same, taking around 21 nanoseconds to format a 32-bit float and
31 nanoseconds to format a 64-bit float.

There is also a Rust-specific benchmark comparing this implementation to the
standard library which you can run with:

```console
$ cargo bench
```

The benchmark shows Ryū approximately 4-10x faster than the standard library
across a range of f32 and f64 inputs. Measurements are in nanoseconds per
iteration; smaller is better.

| type=f32 | 0.0  | 0.1234 | 2.718281828459045 | f32::MAX |
|:--------:|:----:|:------:|:-----------------:|:--------:|
| RYU      | 3ns  | 28ns   | 23ns              | 22ns     |
| STD      | 40ns | 106ns  | 128ns             | 110ns    |

| type=f64 | 0.0  | 0.1234 | 2.718281828459045 | f64::MAX |
|:--------:|:----:|:------:|:-----------------:|:--------:|
| RYU      | 3ns  | 50ns   | 35ns              | 32ns     |
| STD      | 39ns | 105ns  | 128ns             | 202ns    |

## Formatting

This library tends to produce more human-readable output than the standard
library's to\_string, which never uses scientific notation. Here are two
examples:

- *ryu:* 1.23e40, *std:* 12300000000000000000000000000000000000000
- *ryu:* 1.23e-40, *std:* 0.000000000000000000000000000000000000000123

Both libraries print short decimals such as 0.0000123 without scientific
notation.

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-BOOST">Boost Software License 1.0</a> at your
option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
