use rand::{Rng, SeedableRng, StdRng};

type WhiteNoise = Vec<Vec<f64>>;
type Octave = Vec<Vec<f64>>;
type HeightMap = Vec<Vec<f64>>;

fn interpolate(x0: f64, x1: f64, alpha: f64) -> f64 {
    x0 * (1.0 - alpha) + alpha * x1
}

trait Enumerated {
    fn at(&self, x: usize, y: usize) -> f64;
}

trait WithDimensions2D {
    fn with_dimensions(width: usize, height: usize) -> WhiteNoise;
}

trait Sized2D {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
}

impl Enumerated for HeightMap {
    fn at(&self, x: usize, y: usize) -> f64 {
        self[y][x]
    }
}

impl Sized2D for WhiteNoise {
    fn width(&self) -> usize { self.len() }
    fn height(&self) -> usize { self[0].len() }
}

impl WithDimensions2D for WhiteNoise {
    fn with_dimensions(width: usize, height: usize) -> WhiteNoise {
        // TODO handle that width == 0
        // TODO handle that height == 0

        let seed: &[_] = &[0, 0, 0, 0];
        let mut rng: StdRng = SeedableRng::from_seed(seed);
        let mut rows = Vec::with_capacity(width);

        for _ in 0..height {
            let mut cols = Vec::with_capacity(height);

            for _ in 0..width {
                cols.push(rng.gen::<f64>());
            }

            rows.push(cols);
        }

        rows
    }
}

fn smooth_noise(base_noise: &WhiteNoise, octave: u32) -> Octave {
    let width = base_noise.width();
    let height = base_noise.height();

    let period = 1 << octave;
    let frequency = 1.0 / (period as f64);

    let mut rows: Vec<Vec<f64>> = Vec::with_capacity(width);

    for i in 0..width {
        let mut cols = Vec::with_capacity(height);

        let sample_i0: usize = (i / period) * period;
        let sample_i1: usize = (sample_i0 + period) % width;
        let h_blend: f64 = ((i - sample_i0) as f64) * frequency;

        for j in 0..height {
            let sample_j0: usize = (j / period) * period;
            let sample_j1: usize = (sample_j0 + period) % width;
            let v_blend: f64 = ((j - sample_j0) as f64) * frequency;

            let top = interpolate(
                base_noise[sample_i0][sample_j0],
                base_noise[sample_i1][sample_j0],
                h_blend
            );

            let bottom = interpolate(
                base_noise[sample_i0][sample_j1],
                base_noise[sample_i1][sample_j1],
                h_blend
            );

            cols.push(interpolate(top, bottom, v_blend));
        }

        rows.push(cols);
    }

    rows
}

fn blend(base_noise: &WhiteNoise, octaves: &Vec<Octave>) -> HeightMap {
    let width = base_noise.width();
    let height = base_noise.height();

    let persistence = 0.5;

    let mut amp: f64 = 1.0;
    let mut total_amp: f64 = 0.0;
    let mut rows = Vec::with_capacity(width);

    for _ in 0..width {
        let mut cols = Vec::with_capacity(height);
        for _ in 0..height { cols.push(0 as f64); }
        rows.push(cols);
    }

    for octave in octaves {
        amp *= persistence;
        total_amp += amp;

        for i in 0..width {
            for j in 0..height {
                rows[i][j] += octave[i][j] * amp;
            }
        }
    }

    for i in 0..width {
        for j in 0..height {
            rows[i][j] /= total_amp;
        }
    }

    rows
}

pub fn random_octaves(width: usize, height: usize) -> Vec<Octave> {
    let whitenoise = WhiteNoise::with_dimensions(width, height);
    let mut octaves = Vec::with_capacity(7);

    for i in 0..7 {
        octaves.push(smooth_noise(&whitenoise, i));
    }

    octaves.reverse();

    octaves
}

pub fn random_height_map(width: usize, height: usize) -> HeightMap {
    let whitenoise = WhiteNoise::with_dimensions(width, height);
    let mut octaves = Vec::with_capacity(7);

    for i in 0..7 {
        octaves.push(smooth_noise(&whitenoise, i));
    }

    octaves.reverse();

    blend(&whitenoise, &octaves)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_height_map_should_produce_the_right_dimensions() {
        let hm = random_height_map(10, 10);

        assert!(hm.len() == 10);

        for i in 0..10 {
            assert!(hm[i].len() == 10);
        }
    }

    #[test]
    fn from_seed_should_produce_the_right_size() {
        let o = WhiteNoise::with_dimensions(10, 10);

        assert!(o.len() == 10);

        for i in 0..10 {
            assert!(o[i].len() == 10);
        }
    }

    #[test]
    fn from_seed_should_produce_random_numbers_for_each() {
        let o = WhiteNoise::with_dimensions(10, 1);

        let mut all0 = true;

        for i in 0..10 {
            all0 = all0 && o[0][i] == 0.0;
        }

        assert!(!all0);
    }

    #[test]
    fn from_seed_should_produce_random_numbers_between_0_and_1() {
        let o = WhiteNoise::with_dimensions(100, 100);

        for i in 0..100 {
            for j in 0..100 {
                assert!(o[i][j] <= 1.0);
                assert!(o[i][j] >= 0.0);
            }
        }
    }
}
