
pub fn powerset(set: &[i32]) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = vec![];
    for mask in 0..(1 << set.len()) {
        res.push(vec![]);
        for i in 0..set.len() {
            if mask & (1 << i) != 0 {
                res[mask].push(set[i]);
            }
        }
    }   
    res
}