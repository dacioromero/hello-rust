use std::env;

fn main() {
    let max: u64 = env::args().nth(1).unwrap().parse().unwrap();

    let mut f: (u64, u64) = (0, 1);

    println!("{}", f.0);

    for _ in 0..max {
        println!("{}", f.1);
        f = (f.1, f.0 + f.1)
    }

    println!("{}", f.1);
}
