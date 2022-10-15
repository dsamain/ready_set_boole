
pub fn adder(a: u32, b: u32) -> u32 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let tmp = (a & b) << 1;
        a ^= b;
        b = tmp;
    }
    a
}
