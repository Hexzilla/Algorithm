fn calculate(n: usize) -> u32 {
    let mut sum = 1;
    let mut values: [u32; 2] = [0; 2];
    
    values[0] = 0;
    values[1] = 1;
    
    for i in 2..=n {
        let item = values[0] + values[1];
        values[0] = values[1];
        values[1] = item;
        sum += item;
        println!("item {}, {}, {}", i, item, sum);
    }
    return sum;
}

fn main() {
    let n = 40;
    println!("Sum {}", calculate(n));
}
