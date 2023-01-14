use std::io;

fn main() {
    let mut line1 = String::new();
    let _ = io::stdin().read_line(&mut line1);
    let mochi_cnt: u32 = line1.trim().parse().unwrap();

    let mut values: Vec<u32> = vec![];

    for _ in 0..mochi_cnt {
        let mut value = String::new();
        let _ = io::stdin().read_line(&mut value);
        values.push(value.trim().parse().unwrap());
    }

    values.sort();
    values.reverse();

    let mut prev_mochi = values[0];
    let mut cnt: u32 = 1;

    for i in 1..values.len() {
        if values[i] < prev_mochi {
            cnt += 1;
            prev_mochi = values[i];
        }
    }

    println!("{}", cnt);
}
