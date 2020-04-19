use std::env;

fn main() {
    let mut total: i128 = 0;

    for arg in env::args().skip(1) {
        total += arg.parse::<i128>().unwrap();
    }

    println!("The total is {}", total);
}
