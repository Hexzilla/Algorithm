fn calculate() {
    for a in 1..=9 {
        for b in 1..=9 {
            for c in 1..=9 {
                let v1 = a * 100 + b * 10 + a;
                let v2 = c * 100 + b * 10 + c;
                let mul = v1 * v2;
                if mul == 123487 {
                    println!("Found {}, {}", v1, v2)
                }
            }
        }
    }
}

fn main() {
    calculate();
}
