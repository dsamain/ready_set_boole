pub fn map(x: u16, y: u16) -> f64 {
    let ret = (((x as u64) + ((y as u64) << 16))) as f64;
    ret / ((1 as u64) << 32) as f64
} 

pub fn reverse_map(n: f64) -> (u16, u16) {

    let mut x = (n * ((1 as i64) << 32) as f64) as u64; 
    x &= ((1 as u64) << 16) - 1;
    let y = (n * ((1 as i64) << 16) as f64) as u64;
    (x as u16, y as u16)
}