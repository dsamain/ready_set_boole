
use crate::adder::*;

pub fn multiplier(a: u32, b: u32) -> u32 {
    let mut res: u32 = 0;
    for i in 0..32 {
        if b >> i & 1 == 0 {
            continue;
        }
        res = adder(res, a << i);
    }
    res
}