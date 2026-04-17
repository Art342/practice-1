pub fn get_total_x(a: &[i32], b: &[i32]) -> i32 {
    let mut count = 0;
    let max_a = *a.iter().max().unwrap();
    let min_b = *b.iter().min().unwrap();

    for x in max_a..=min_b {
        let is_multiple_of_a = a.iter().all(|&item| x % item == 0);
        let is_divisor_of_b = b.iter().all(|&item| item % x == 0);

        if is_multiple_of_a && is_divisor_of_b {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_total_x() {
        assert_eq!(get_total_x(&[2, 4], &[16, 32, 96]), 3);
        assert_eq!(get_total_x(&[3, 4], &[24, 48]), 2);
    }
}