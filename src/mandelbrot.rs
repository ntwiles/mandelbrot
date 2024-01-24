

pub struct Mandelbrot {
    diverge_iterations: u32, 
    diverge_threshold: f64,
    width: u32,
    height: u32,
}

impl Mandelbrot {
    pub fn new(width: u32, height: u32, diverge_iterations: u32, diverge_threshold: f64) -> Self {
        Self {
            diverge_iterations,
            diverge_threshold,
            width,
            height,
        }
    }

    // TODO: Put these coords in the correct space before running this fn, so it
    // doesn't have to do the conversion.
    pub fn calculage_diverge_number(&self, a: u32, b: u32) -> Option<u32> {
        let mut aa = 0.0;
        let mut bb = 0.0;

        for n in 0..self.diverge_iterations {
            let xtemp = aa * aa - bb * bb + (a as f64 / self.width as f64) * 2.0 - 1.5;

            bb = 2.0 * aa * bb + (b as f64 / self.height as f64) * 2.0 - 1.0;
            aa = xtemp;

            if aa * aa + bb * bb > self.diverge_threshold {
                return Some(n);
            }
        }

        None
    }
}