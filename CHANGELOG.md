# CHANGELOG

# [# 0.2.0 (2020-07-14) - ECMAScript compliant `f32` conversions Release](https://github.com/boa-dev/ryu-js/compare/v0.1.0...v0.2.0)

Feature enhancements:

 - [FEATURE #6](https://github.com/boa-dev/ryu-js/pull/6):
  ECMAScript specification complaint `f32` to string conversions. (@HalidOdat)

Bug fixes:

 - [BUG #2](https://github.com/boa-dev/ryu-js/pull/2):
  Fixed Comatibility with rust `1.31.0`. (@HalidOdat)
 - [BUG #2](https://github.com/boa-dev/ryu-js/pull/2):
  Fixed converting from `-0.0` to `0`. (@HalidOdat)
 - [BUG #2](https://github.com/boa-dev/ryu-js/pull/2):
  Fixed max length docs for `format32` and `format64`. (@HalidOdat)

Internal improvements:

 - [INTERNAL #2](https://github.com/boa-dev/ryu-js/pull/2):
  Optimized `0` and `-0` to string conversion (@HalidOdat)

# 0.1.0 (2020-07-13) - ECMAScript complaint `f64` conversions Release

This is the initial release of this crate, it introduces ECMAScript compliant `f64` to string conversions.

Feature enhancements:

- [FEATURE](https://github.com/boa-dev/ryu-js/commit/ed781f5772882e38c53d40707a60b4f11414b9c8):
  ECMAScript specification complaint `f64` to string conversions. (@Tropid)
- [FEATURE](https://github.com/boa-dev/ryu-js/commit/fe366fa397d04324fa693b5d85134851b09719b3):
  Change name from `ryu` to `ryu-js`. (@Tropid)

Bug fixes:

- [BUG #1](https://github.com/boa-dev/ryu-js/pull/1):
  Fixed buffer overflow with length greater than 24 (max is 25). (@HalidOdat)

Internal improvements:

 - [INTERNAL #1](https://github.com/boa-dev/ryu-js/pull/2):
  Fixed all clippy warnings/errors and tests (@HalidOdat)
