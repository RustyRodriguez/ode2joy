pub fn euler<F>(f: F, y0: f64, t0: f64, t_end: f64, step: f64) -> Vec<(f64, f64)>
where
    F: Fn(f64, f64) -> f64,
{
    let mut results = Vec::new();
    let mut t = t0;
    let mut y = y0;

    while t <= t_end {
        results.push((t, y));
        y += step * f(t, y);
        t += step;
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euler_zero_derivative() {
        let f = |_t: f64, _y: f64| 0.0;
        let result = euler(f, 5.0, 0.0, 1.0, 0.1);

        for &(_, y) in &result {
            assert_eq!(y, 5.0);
        }
    }

    #[test]
    fn test_euler_linear_decay() {
        let f = |_t: f64, y: f64| -y;
        let result = euler(f, 1.0, 0.0, 1.0, 0.1);
        let expected = 1.0 / std::f64::consts::E;
        let approx = result.last().unwrap().1;

        assert!((approx - expected).abs() < 0.1, "Expected {}, actual {}", expected, approx);
    }
}