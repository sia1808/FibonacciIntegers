use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();
    let k: u32 = input.parse().expect("Not a good number!");
    let mut sum: u32 = 0;
    for i in 0..=k{
        sum = i*i + sum;
    }
    println!("when k = {}, sum = {}", k, sum);
}
