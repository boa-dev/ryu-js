use core::ptr;

use digit_table::*;

#[cfg_attr(feature = "no-panic", inline)]
pub unsafe fn write_exponent3(mut k: isize, mut result: *mut u8) -> usize {
    let sign = k < 0;
    if sign {
        *result = b'-';
        result = result.offset(1);
        k = -k;
    }

    if k >= 100 {
        *result = b'0' + (k / 100) as u8;
        k %= 100;
        let d = DIGIT_TABLE.get_unchecked(k as usize * 2);
        ptr::copy_nonoverlapping(d, result.offset(1), 2);
        sign as usize + 3
    } else if k >= 10 {
        let d = DIGIT_TABLE.get_unchecked(k as usize * 2);
        ptr::copy_nonoverlapping(d, result, 2);
        sign as usize + 2
    } else {
        *result = b'0' + k as u8;
        sign as usize + 1
    }
}

#[cfg_attr(feature = "no-panic", inline)]
pub unsafe fn write_exponent2(mut k: isize, mut result: *mut u8) -> usize {
    let sign = k < 0;
    if sign {
        *result = b'-';
        result = result.offset(1);
        k = -k;
    }

    if k >= 10 {
        let d = DIGIT_TABLE.get_unchecked(k as usize * 2);
        ptr::copy_nonoverlapping(d, result, 2);
        sign as usize + 2
    } else {
        *result = b'0' + k as u8;
        sign as usize + 1
    }
}
