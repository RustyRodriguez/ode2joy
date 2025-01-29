pub struct ODESolver;

impl ODESolver {

    pub fn euler<F>(f: F, y0: f64, t0: f64, steps: f64, step_size: f64) -> Result<f64, &'static str>
    where F: Fn(f64, f64) -> f64,
    {
        if step_size == 0.0 {
            return Err("Step size must be positive");
        }
        let mut y = y0;
        let mut t = t0;

        while t < steps {
            y += step_size * f(t, y);
            t += step_size;
        }

        Ok(y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euler() {
        let dydt = |t: f64, y: f64| -2.0 * y;
        let solution = ODESolver::euler(dydt, 1.0, 0.0, 1.0, 0.01);

        let y_final = solution.unwrap();
        assert!((y_final - (-1.0 * 1.0f64.exp()).exp()).abs() < 0.1);
    }
}