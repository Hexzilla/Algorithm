fn gcd(a: u32, b: u32) -> u32 {
    if a == 0 {
        return b;
    }
    return gcd(b % a, a);
}

fn main() {
    println!("GCD(16, 28) = {}", gcd(16, 28));
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(gcd(16, 28), 4);
        assert_eq!(gcd(35, 15), 5);
    }
}