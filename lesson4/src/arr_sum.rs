pub fn sum(arr: &[u32]) -> Option<u32> {
    let mut sum:u32 = 0;
    for i in arr.iter() {
        let result = sum.checked_add(*i);
        if result.is_none() {
            return None;
        } else {
            sum = result.unwrap();
        }
    }
    Some(sum)
}