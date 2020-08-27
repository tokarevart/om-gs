pub use std::ops::Range;

pub fn search(range: Range<f64>, eps: f64, f: impl Fn(f64) -> f64) -> f64 {
    assert!(eps > 0.0);

    let invphi = 2.0 / (1.0 + 5.0f64.sqrt());
    let invphi2 = invphi.powi(2);

    let Range{ mut start, mut end } = range;
    let mut x1 = start + invphi2 * (end - start);
    let mut x2 = start + invphi * (end - start);
    let mut f1 = f(x1);
    let mut f2 = f(x2);
    while eps < end - start {
        if f1 < f2 {
            end = x2;
            x2 = x1;
            f2 = f1;
            x1 = start + invphi2 * (end - start);
            f1 = f(x1);
        } else {
            start = x1;
            x1 = x2;
            f1 = f2;
            x2 = start + invphi * (end - start);
            f2 = f(x2);
        }
    }

    0.5 * (start + end)
}

pub fn search_with_n(range: Range<f64>, n: usize, f: impl Fn(f64) -> f64) -> f64 {
    assert!(n > 0);

    let invphi = 2.0 / (1.0 + 5.0f64.sqrt());
    let invphi2 = invphi.powi(2);

    let Range{ mut start, mut end } = range;
    let mut x1 = start + invphi2 * (end - start);
    let mut x2 = start + invphi * (end - start);
    let mut f1 = f(x1);
    let mut f2 = f(x2);
    for _ in 0..n {
        if f1 < f2 {
            end = x2;
            x2 = x1;
            f2 = f1;
            x1 = start + invphi2 * (end - start);
            f1 = f(x1);
        } else {
            start = x1;
            x1 = x2;
            f1 = f2;
            x2 = start + invphi * (end - start);
            f2 = f(x2);
        }
    }

    0.5 * (start + end)
}
