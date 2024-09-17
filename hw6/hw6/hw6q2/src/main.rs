fn main() {
    let mut f = [0u128; 101];
    f[0] = 0;
    f[1] = 1;


    for i in 2..=100 {
        f[i] = f[i-2] + f[i-1];
    }


    for i in 0..=100 {
        println!("F[{:?}] = {:?}", i, f[i]);
    }
}
