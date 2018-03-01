fn calculate(n: i32) -> i32 {
    let mut sum: i32 = 0;
    let mut sign: i32 = -1;
    for i in 0..n {
        sum += sign * 3 * (i + 1);
        sign *= -1;
    }
    return sum;
}

fn main() {
    println!("Result {}", calculate(15));
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(calculate(2), 3);
        assert_eq!(calculate(3), -6);
        assert_eq!(calculate(4), 6);
        assert_eq!(calculate(5), -9);
        assert_eq!(calculate(6), 9);
        assert_eq!(calculate(7), -12);
        assert_eq!(calculate(8), 12);
        assert_eq!(calculate(9), -15);
        assert_eq!(calculate(10), 15);
    }
}