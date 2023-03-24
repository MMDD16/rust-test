fn sum_u32(values: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0;
    for &value in values {
        match sum.checked_add(value) {
            Some(new_sum) => sum = new_sum,
            None => return None,
        }
    }
    Some(sum)
}
fn main() {
    let values = [5,8,10,11,6];
    match sum_u32(&values) {
        Some(sum) => println!("Sum of values: {}", sum),
        None => println!("Sum of values overflows!"),
    }
}
