extern "C" {
    fn sqrt(x: f64) -> f64;
}

fn main() {
    let x = 5.0;
    let q: f64;
    unsafe {
        q = sqrt(x);
    }
    println!("Sqrt root of {} is {}", x, q);
}
