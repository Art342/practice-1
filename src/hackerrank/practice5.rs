pub fn count_apples_and_oranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) -> (i32, i32) {
    let apple_count = apples.iter()
        .filter(|&&d| a + d >= s && a + d <= t)
        .count() as i32;

    let orange_count = oranges.iter()
        .filter(|&&d| b + d >= s && b + d <= t)
        .count() as i32;

    (apple_count, orange_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apples_oranges() {
        // Тест із умови задачі: будинок 7-11, дерева 5 і 15, фрукти впали на відстані...
        let (app, org) = count_apples_and_oranges(7, 11, 5, 15, &[-2, 2, 1], &[5, -6]);
        assert_eq!(app, 1);
        assert_eq!(org, 1);
    }
}