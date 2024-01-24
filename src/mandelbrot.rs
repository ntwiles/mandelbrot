pub struct Mandelbrot {
    diverge_iterations: u32, 
    diverge_threshold: f64,
}

impl Mandelbrot {
    pub fn new(diverge_iterations: u32, diverge_threshold: f64) -> Self {
        Self {
            diverge_iterations,
            diverge_threshold,
        }
    }

    pub fn calculage_diverge_number(&self, a: f64, b: f64) -> Option<u32> {
        let mut aa = 0.0;
        let mut bb = 0.0;

        for n in 0..self.diverge_iterations {
            let xtemp = aa * aa - bb * bb + a;

            bb = 2.0 * aa * bb + b;
            aa = xtemp;

            if aa * aa + bb * bb > self.diverge_threshold {
                return Some(n);
            }
        }

        None
    }
}