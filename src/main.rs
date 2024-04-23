fn simpsons_integration<F>(f: F, a: f64, b: f64, n: usize) -> f64
    where
        F: Fn(f64) -> f64,
{
    let h = (b - a) / n as f64;
    (0..n)
        .map(|i| {
            let x0 = a + i as f64 * h;
            let x1 = x0 + h / 2.0;
            let x2 = x0 + h;
            (h / 6.0) * (f(x0) + 4.0 * f(x1) + f(x2))
        })
        .sum()
}

fn calculate_rc<F>(f4: F, a: f64, b: f64, n: usize) -> f64
    where F: Fn(f64) -> f64 {
    let h = (b - a) / n as f64;
    let factor = (b - a) * h.powi(4) / 180.0;
    let bruh: f64 = (0..n)
        .map(|i| {
            let x = a + i as f64 * h;
            f4(x).abs()
        })
        .reduce(f64::max).unwrap();
    return factor * bruh;
}

fn main() {
    let f = |x: f64| 1.0 / (x.powi(2) * x.ln().powi(2));
    let f4 = |x: f64| -1.0 * (24.0 * x.ln().powi(3) + 52.0 * x.ln().powi(2) + 54.0 * x.ln() + 24.0) / (x.powi(5) * x.ln().powi(5));
    let a = 2.0;
    let b = 8.0;
    let n = 50;
    let result = simpsons_integration(f, a, b, n);
    let rc = calculate_rc(f4, a, b, n);
    println!("Вычисленный результат: {}", result);
    println!("Верхняя граница для остаточного члена: {}", rc);
}