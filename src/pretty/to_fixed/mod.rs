use crate::{
    d2s::{DOUBLE_BIAS, DOUBLE_EXPONENT_BITS, DOUBLE_MANTISSA_BITS},
    digit_table::DIGIT_TABLE,
    pretty::{
        format64,
        to_fixed::d2fixed_full_table::{
            ADDITIONAL_BITS_2, MIN_BLOCK_2, POW10_OFFSET, POW10_OFFSET_2, POW10_SPLIT,
            POW10_SPLIT_2,
        },
    },
};
mod d2fixed_full_table;

const POW10_ADDITIONAL_BITS: u32 = 120;

/// Returns `floor(log_10(2^e))` requires `0 <= e <= 1650`.
fn log10_pow2(e: i32) -> u32 {
    // The first value this approximation fails for is 2^1651 which is just greater than 10^297.
    assert!((0..=1650).contains(&e));

    ((e as u32) * 78913) >> 18
}

fn index_for_exponent(e: u32) -> u32 {
    (e + 15) / 16
}

fn pow10_bits_for_index(idx: u32) -> u32 {
    16 * idx + POW10_ADDITIONAL_BITS
}

fn length_for_index(idx: u32) -> u32 {
    // +1 for ceil, +16 for mantissa, +8 to round up when dividing by 9
    (log10_pow2(16 * idx as i32) + 1 + 16 + 8) / 9
}

fn umul256(a: u128, b_hi: u64, b_lo: u64) -> (u128, u128) {
    let a_lo = a as u64;
    let a_hi = (a >> 64) as u64;

    let b00 = (a_lo as u128) * (b_lo as u128);
    let b01 = (a_lo as u128) * (b_hi as u128);
    let b10 = (a_hi as u128) * (b_lo as u128);
    let b11 = (a_hi as u128) * (b_hi as u128);

    let b00_lo = b00 as u64;
    let b00_hi = (b00 >> 64) as u64;

    let mid1 = b10 + b00_hi as u128;
    let mid1_lo = (mid1) as u64;
    let mid1_hi = (mid1 >> 64) as u64;

    let mid2 = b01 + mid1_lo as u128;
    let mid2_lo = (mid2) as u64;
    let mid2_hi = (mid2 >> 64) as u64;

    let p_hi = b11 + mid1_hi as u128 + mid2_hi as u128;
    let p_lo = ((mid2_lo as u128) << 64) | b00_lo as u128;

    (p_hi, p_lo)
}

// Returns the high 128 bits of the 256-bit product of a and b.
fn umul256_hi(a: u128, b_hi: u64, b_lo: u64) -> u128 {
    // Reuse the umul256 implementation.
    // Optimizers will likely eliminate the instructions used to compute the
    // low part of the product.
    let (hi, _lo) = umul256(a, b_hi, b_lo);
    hi
}

// Unfortunately, gcc/clang do not automatically turn a 128-bit integer division
// into a multiplication, so we have to do it manually.
fn uint128_mod1e9(v: u128) -> u32 {
    // After multiplying, we're going to shift right by 29, then truncate to uint32_t.
    // This means that we need only 29 + 32 = 61 bits, so we can truncate to uint64_t before shifting.
    let multiplied = umul256_hi(v, 0x89705F4136B4A597, 0x31680A88F8953031) as u64;

    // For uint32_t truncation, see the mod1e9() comment in d2s_intrinsics.rs
    let shifted = (multiplied >> 29) as u32;

    (v as u32).wrapping_sub(1000000000u32.wrapping_mul(shifted))
}

// Best case: use 128-bit type.
fn mul_shift_mod1e9(m: u64, mul: &[u64; 3], j: i32) -> u32 {
    let b0 = m as u128 * mul[0] as u128; // 0
    let b1 = m as u128 * mul[1] as u128; // 64
    let b2 = m as u128 * mul[2] as u128; // 128

    assert!((128..=180).contains(&j));

    let mid = b1 + ((b0 >> 64) as u64) as u128; // 64
    let s1 = b2 + ((mid >> 64) as u64) as u128; // 128
    uint128_mod1e9(s1 >> (j - 128))
}

/// Convert `digits` to decimal and write the last 9 decimal digits to result.
/// If `digits` contains additional digits, then those are silently ignored.
unsafe fn append_nine_digits(mut digits: u32, result: *mut u8) {
    debug_assert!(!result.is_null());

    if digits == 0 {
        result.write_bytes(b'0', 9);
        // memset(result, '0', 9);
        return;
    }

    //   for (uint32_t i = 0; i < 5; i += 4) {
    for i in (0u32..5).step_by(4) {
        let c = digits % 10000;
        digits /= 10000;
        let c0 = (c % 100) << 1;
        let c1 = (c / 100) << 1;

        result
            .offset(7 - i as isize)
            .copy_from_nonoverlapping(DIGIT_TABLE.as_ptr().offset(c0 as isize), 2);
        result
            .offset(5 - i as isize)
            .copy_from_nonoverlapping(DIGIT_TABLE.as_ptr().offset(c1 as isize), 2);
        // memcpy(result + 7 - i, DIGIT_TABLE + c0, 2);
        // memcpy(result + 5 - i, DIGIT_TABLE + c1, 2);
    }
    *(result.offset(0)) = b'0' + digits as u8;
}

/// Convert `digits` to a sequence of decimal digits. Append the digits to the result.
/// The caller has to guarantee that:
///   10^(olength-1) <= digits < 10^olength
/// e.g., by passing `olength` as `decimalLength9(digits)`.
unsafe fn append_n_digits(olength: u32, mut digits: u32, result: *mut u8) {
    debug_assert!(!result.is_null());

    let mut i = 0;
    while digits >= 10000 {
        let c = digits % 10000;

        digits /= 10000;
        let c0 = (c % 100) << 1;
        let c1 = (c / 100) << 1;
        result
            .offset(olength as isize - i as isize - 2)
            .copy_from_nonoverlapping(DIGIT_TABLE.as_ptr().offset(c0 as isize), 2);
        result
            .offset(olength as isize - i as isize - 4)
            .copy_from_nonoverlapping(DIGIT_TABLE.as_ptr().offset(c1 as isize), 2);
        // memcpy(result + olength - i - 2, DIGIT_TABLE + c0, 2);
        // memcpy(result + olength - i - 4, DIGIT_TABLE + c1, 2);
        i += 4;
    }
    if digits >= 100 {
        let c = (digits % 100) << 1;
        digits /= 100;
        result
            .offset(olength as isize - i as isize - 2)
            .copy_from_nonoverlapping(DIGIT_TABLE.as_ptr().offset(c as isize), 2);
        // memcpy(result + olength - i - 2, DIGIT_TABLE + c, 2);
        i += 2;
    }
    if digits >= 10 {
        let c = digits << 1;
        result
            .offset(olength as isize - i as isize - 2)
            .copy_from_nonoverlapping(DIGIT_TABLE.as_ptr().offset(c as isize), 2);
        // memcpy(result + olength - i - 2, DIGIT_TABLE + c, 2);
    } else {
        *result = b'0' + digits as u8;
    }
}

/// Convert `digits` to decimal and write the last `count` decimal digits to result.
/// If `digits` contains additional digits, then those are silently ignored.
unsafe fn append_c_digits(count: u32, mut digits: u32, result: *mut u8) {
    debug_assert!(!result.is_null());

    // Copy pairs of digits from DIGIT_TABLE.
    let mut i: u32 = 0;
    //   for (; i < count - 1; i += 2) {
    while i < count - 1 {
        let c: u32 = (digits % 100) << 1;
        digits /= 100;
        result
            .offset((count - i - 2) as isize)
            .copy_from_nonoverlapping(DIGIT_TABLE.as_ptr().offset(c as isize), 2);
        // memcpy(result + count - i - 2, DIGIT_TABLE + c, 2);

        i += 2;
    }
    // Generate the last digit if count is odd.
    if i < count {
        let c = b'0' + (digits % 10) as u8;
        *result.offset((count - i - 1) as isize) = c;
        // result[count - i - 1] = c;
    }
}

// Returns the number of decimal digits in v, which must not contain more than 9 digits.
fn decimal_length9(v: u32) -> u32 {
    // Function precondition: v is not a 10-digit number.
    // (f2s: 9 digits are sufficient for round-tripping.)
    // (d2fixed: We print 9-digit blocks.)
    debug_assert!(v < 1000000000);

    if v >= 100000000 {
        9
    } else if v >= 10000000 {
        8
    } else if v >= 1000000 {
        7
    } else if v >= 100000 {
        6
    } else if v >= 10000 {
        5
    } else if v >= 1000 {
        4
    } else if v >= 100 {
        3
    } else if v >= 10 {
        2
    } else {
        1
    }
}

// FIXME: remove after prototyping
#[allow(clippy::missing_safety_doc)]
#[must_use]
#[cfg_attr(feature = "no-panic", no_panic)]
pub unsafe fn format64_to_fixed(f: f64, fraction_digits: u8, result: *mut u8) -> usize {
    // SKIPPED: 1. Let x be ? thisNumberValue(this value).
    // SKIPPED: 2. Let f be ? ToIntegerOrInfinity(fractionDigits).
    // SKIPPED: 3. Assert: If fractionDigits is undefined, then f is 0.
    // SKIPPED: 4. If f is not finite, throw a RangeError exception.
    // 5. If f < 0 or f > 100, throw a RangeError exception.
    debug_assert!((0..=100).contains(&fraction_digits));

    // 10. If x ≥ 10^21, then
    let f_abs = if f < 0.0 { -f } else { f };
    if f_abs >= 1e21 {
        // a. Let m be ! ToString(𝔽(x)).
        return format64(f, result);
    }

    debug_assert!(!result.is_null());
    let bits = f.to_bits();
    let sign = ((bits >> (DOUBLE_MANTISSA_BITS + DOUBLE_EXPONENT_BITS)) & 1) != 0;
    let ieee_mantissa = bits & ((1u64 << DOUBLE_MANTISSA_BITS) - 1);
    let ieee_exponent =
        (bits >> DOUBLE_MANTISSA_BITS) as u32 & ((1u32 << DOUBLE_EXPONENT_BITS) - 1);

    // Special case when it's 0 or -0 it's the same.
    //
    // Return and append '.' and '0's is needed.
    //
    // See: https://tc39.es/ecma262/#%E2%84%9D
    if ieee_exponent == 0 && ieee_mantissa == 0 {
        *result = b'0';
        if fraction_digits == 0 {
            return 1;
        }

        *result.offset(1) = b'.';
        for offset in 2..(2 + fraction_digits as isize) {
            *result.offset(offset) = b'0';
        }
        return 2 + fraction_digits as usize;
    }

    // This assertion should not fail, because if the abs(f) >= 1e21 check above.
    //
    // See tests.
    const MAX_EXPONENT: u32 = 0b100_0100_0100; // 1029
    debug_assert!((0..=MAX_EXPONENT).contains(&ieee_exponent));

    let mut index = 0isize;
    if sign {
        *result = b'-';
        index += 1;
    }

    let (e2, m2) = if ieee_exponent == 0 {
        (1 - DOUBLE_BIAS - DOUBLE_MANTISSA_BITS as i32, ieee_mantissa)
    } else {
        (
            ieee_exponent as i32 - DOUBLE_BIAS - DOUBLE_MANTISSA_BITS as i32,
            (1 << DOUBLE_MANTISSA_BITS) | ieee_mantissa,
        )
    };

    const MAX_E2: i32 = MAX_EXPONENT as i32 - DOUBLE_BIAS - DOUBLE_MANTISSA_BITS as i32;
    debug_assert!((..=MAX_E2).contains(&e2));

    let mut nonzero = false;

    // Write the whole part (integral part) of the floating point.
    //
    // xxxxxxx.1234567 (write xs)
    if e2 >= -(DOUBLE_MANTISSA_BITS as i32) {
        let idx = if e2 < 0 {
            0
        } else {
            index_for_exponent(e2 as u32)
        };
        let p10bits = pow10_bits_for_index(idx);
        let len = length_for_index(idx) as i32;

        for i in (0..=(len - 1)).rev() {
            let j = p10bits as i32 - e2;
            // Temporary: j is usually around 128, and by shifting a bit, we push it to 128 or above, which is
            // a slightly faster code path in mulShift_mod1e9. Instead, we can just increase the multipliers.
            let digits = mul_shift_mod1e9(
                m2 << 8,
                &POW10_SPLIT[POW10_OFFSET[idx as usize] as usize + i as usize],
                j + 8,
            );
            if nonzero {
                append_nine_digits(digits, result.offset(index));
                index += 9;
            } else if digits != 0 {
                let olength = decimal_length9(digits);
                append_n_digits(olength, digits, result.offset(index));
                index += olength as isize;
                nonzero = true;
            }
        }
    }

    // If the whole part is zero (nothing was writen), write a zero.
    if !nonzero {
        *result.offset(index) = b'0';
        index += 1;
    }

    // If fraction_digits is not zero, then write the dot.
    if fraction_digits != 0 {
        *result.offset(index) = b'.';
        index += 1;
    }

    // Check if it has fractional part.
    if e2 >= 0 {
        result
            .offset(index)
            .write_bytes(b'0', fraction_digits as usize);
        index += fraction_digits as isize;
        return index as usize;
    }

    let mut round_up = false;

    let fraction_digits = fraction_digits as u32;

    let idx = -e2 / 16;
    let min_block_2 = MIN_BLOCK_2[idx as usize];

    // fraction_digits is defined to be [0, 100] inclusive.
    //
    // Therefore blocks can be [1, 12] inclusive.

    let blocks: u32 = fraction_digits / 9 + 1;
    let i = if blocks <= min_block_2 as u32 {
        result
            .offset(index)
            .write_bytes(b'0', fraction_digits as usize);
        // memset(result + index, '0', precision);
        index += fraction_digits as isize;

        blocks
    } else {
        0
    };

    // domain of loop i = [0, 11] inclusive
    for i in i..blocks {
        let p: isize = POW10_OFFSET_2[idx as usize] as isize + i as isize - min_block_2 as isize;
        if p >= POW10_OFFSET_2[idx as usize + 1] as isize {
            // If the remaining digits are all 0, then we might as well use memset.
            // No rounding required in this case.
            let fill = fraction_digits as usize - 9 * i as usize;
            // memset(result + index, '0', fill);
            result.offset(index).write_bytes(b'0', fill);
            index += fill as isize;
            break;
        }

        // Temporary: j is usually around 128, and by shifting a bit, we push it to 128 or above, which is
        // a slightly faster code path in mulShift_mod1e9. Instead, we can just increase the multipliers.
        let j: isize = ADDITIONAL_BITS_2 as isize + (-(e2 as isize) - 16 * idx as isize);
        let mut digits: u32 = mul_shift_mod1e9(m2 << 8, &POW10_SPLIT_2[p as usize], j as i32 + 8);

        if i < blocks - 1 {
            append_nine_digits(digits, result.offset(index));
            index += 9;
        } else {
            let maximum: u32 = fraction_digits - 9 * i;
            let mut last_digit: u32 = 0;
            for _k in 0..(9 - maximum) {
                last_digit = digits % 10;
                digits /= 10;
            }

            // If last digit is 5 or above, round up.
            round_up = last_digit >= 5;

            if maximum > 0 {
                append_c_digits(maximum, digits, result.offset(index));
                index += maximum as isize;
            }
            break;
        }
    }

    if round_up {
        let mut round_index: isize = index;
        let mut dot_index: isize = 0; // '.' can't be located at index 0
        loop {
            round_index -= 1;
            let c = *result.offset(round_index);
            if round_index == -1 || c == b'-' {
                *result.offset(round_index + 1) = b'1';
                if dot_index > 0 {
                    *result.offset(dot_index) = b'0';
                    *result.offset(dot_index + 1) = b'.';
                }
                *result.offset(index) = b'0';
                index += 1;
                break;
            }
            if c == b'.' {
                dot_index = round_index;
                continue;
            } else if c == b'9' {
                *result.offset(round_index) = b'0';
                // round_up = true;
                continue;
            } else {
                // if round_up == 2 && c % 2 == 0 {
                //     break;
                // }
                *result.offset(round_index) = c + 1;
                break;
            }
        }
    }

    index as usize
}
