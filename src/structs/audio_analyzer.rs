use rustfft::{FftPlanner, num_complex::Complex};

pub struct AudioAnalyzer {
    fft: std::sync::Arc<dyn rustfft::Fft<f32>>,
    buffer: Vec<Complex<f32>>,
    window_size: usize,
}

impl AudioAnalyzer {
    pub fn new(window_size: usize) -> Self {
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_forward(window_size);
        
        AudioAnalyzer {
            fft,
            buffer: vec![Complex::new(0.0, 0.0); window_size],
            window_size,
        }
    }

    pub fn process(&mut self, samples: &[f32]) -> Vec<f32> {
        // Apply window function and convert to complex
        for (i, &sample) in samples.iter().take(self.window_size).enumerate() {
            // Hann window function
            let window = 0.5 * (1.0 - f32::cos(2.0 * std::f32::consts::PI * i as f32 / self.window_size as f32));
            self.buffer[i] = Complex::new(sample * window, 0.0);
        }

        // Create output buffer
        let mut output = self.buffer.clone();
        
        // Perform FFT
        self.fft.process(&mut output);

        // Convert to magnitude spectrum
        output.iter()
            .take(self.window_size / 2)
            .map(|c| (c.norm() / self.window_size as f32).sqrt() * 2.0)  // Scale the output
            .collect()
    }
}