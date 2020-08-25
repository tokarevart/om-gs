use om_gs::*;

fn main() {
    let range = 0.0..1.0;
    let eps = 1e-4;
    let f = |x| x * x;

    let x = search(range, eps, f);
    println!("  x : {}", x);
    println!("f(x): {}", f(x));
}
