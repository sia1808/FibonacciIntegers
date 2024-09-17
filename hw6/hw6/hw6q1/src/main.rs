fn fib(k:u32) -> u128 {
    if k == 0 {
        return 0;
    }
    if k == 1 {
        return 1;
    }
    else {
        return fib(k-2) + fib(k-1);
    }
        
}

use std::time::SystemTime;

fn main(){
    let mut k = 0;
    let before = SystemTime::now();
    while k <= 50{
        println!("When K is {}, the fib number F(k) is {},", k, fib(k));
        k = k + 1;
        let after = SystemTime::now();
        let difference = after.duration_since(before);
        let difference = difference.expect("Did the clock go back?");
        println!("Time it took: {:?}", difference);
    }
}
