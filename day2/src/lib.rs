pub fn is_safe_report(numbers: &Vec<i32>) -> bool {
    if numbers.len() < 2 {
        return true;
    }

    let is_increasing: bool = numbers[1] > numbers[0];

    numbers.windows(2).all(|pair| {
        let diff = (pair[1] - pair[0]).abs();
        let correct_direction = if is_increasing {
            pair[1] > pair[0]
        } else {
            pair[1] < pair[0]
        };
        correct_direction && diff >= 1 && diff <= 3
    })
}
