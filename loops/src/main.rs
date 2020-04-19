fn main() {
    let n = 8;
    let mut total = 0;
    let mut last = 0;
    for _number in 1..n {
        if total == 0 {
            total = 1;
        }
        let last_total = total;
        total = total + last;
        last = last_total;
    }
    println!("{}", total);
}
