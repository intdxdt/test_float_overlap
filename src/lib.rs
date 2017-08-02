extern crate double_bits as db;
extern crate bit_twiddle as bits;

use bits::{count_trailing_zeros, log2};
use db::{fraction, denormalized, exponent};

fn tz(f: &[u32]) -> i32 {
    if f[0] != 0 {
        count_trailing_zeros(f[0]) as i32
    } else if f[1] != 0 {
        32 + (count_trailing_zeros(f[1]) as i32)
    } else {
        0
    }
}

fn lz(f: &[u32]) -> i32 {
    if f[1] != 0 {
        20i32 - (log2(f[1]) as i32)
    } else if f[0] != 0 {
        52i32 - (log2(f[0]) as i32)
    } else {
        52i32
    }
}

fn lo(n: f64) -> i32 {
    let e = exponent(n);
    let f = fraction(n);
    let z = tz(&f);
    e - (52 - z)
}

fn hi(n: f64) -> i32 {
    if denormalized(n) {
        -(1023 + lz(&fraction(n)))
    } else {
        exponent(n)
    }
}

pub fn test_overlap(a: f64, b: f64) -> bool {
    let (mut a, mut b) = (a, b);
    if b.abs() > a.abs() {
        let t = a;
        a = b;
        b = t;
    }
    if a == 0.0 || b == 0.0 {
        return false;
    }
    let a0 = hi(a);
    let a1 = lo(a);
    let b0 = hi(b);
    let b1 = lo(b);
    (b1 <= a0) && (a1 <= b0)
    //[a1------a0]
    //     [b1-----b0]
    //---------or----------
    //    [a1-------a0]
    //[b1-------b0]
}


#[cfg(test)]
mod float_overlap_test {
    use super::test_overlap;

    #[test]
    fn test_test_overlap() {
        assert!( test_overlap(1.5, 0.5));
        assert!( test_overlap(2f64.powi(-52), 1.0 + 2f64.powi(-52)));
        assert!(!test_overlap(1.0, 0.5));

        //Test 0
        assert!(!test_overlap(0.0, 1.0));
        assert!(!test_overlap(0.0, 0.0));

        //test denormalized numbers
        //
        //underflow - in rust : f64::MIN_EXP == -1021
        //
        //assert!(!test_overlap(2f64.powi(-1024), 2f64.powi(-1023)));
        //assert!(!test_overlap(2f64.powi(-1023), 2f64.powi(-1022)));
        //assert!(test_overlap( 2f64.powi(-1024) + 2f64.powi(-1040), 2f64.powi(-1030)));
        //assert!(!test_overlap(2f64.powi(-1030) - 2f64.powi(-1031), 2f64.powi(-1030)));
    }
}