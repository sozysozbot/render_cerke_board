struct Noise {
    width: usize,
    height: usize,
    data: Vec<Vec<f64>>,
}

use rand::distributions::{Distribution, Uniform};

impl Noise {
    fn gen_noise(width: usize, height: usize) -> Noise {
        /* algorithm taken from https://lodev.org/cgtutor/randomnoise.html#Wood */
        let between = Uniform::from(0.0..1.0);
        let mut rng = rand::thread_rng();
        let mut noise: Vec<Vec<f64>> = Vec::new();
        for _ in 0..height {
            let mut vec = Vec::new();
            for _ in 0..width {
                vec.push(between.sample(&mut rng));
            }
            noise.push(vec);
        }

        Noise {
            width,
            height,
            data: noise,
        }
    }

    fn sample_smooth_noise(&self, x: f64, y: f64) -> f64 {
        /* algorithm taken from https://lodev.org/cgtutor/randomnoise.html#Wood */
        let fract_x = x.fract();
        let fract_y = y.fract();
        let width = self.width;
        let height = self.height;

        //wrap around
        let x1: usize = ((x as usize) + width) % width;
        let y1: usize = ((y as usize) + height) % height;

        //neighbor values
        let x2: usize = (x1 + width - 1) % width;
        let y2: usize = (y1 + height - 1) % height;

        //smooth the noise with bilinear interpolation
        let mut value = 0.0;
        value += fract_x * fract_y * self.data[y1][x1];
        value += (1. - fract_x) * fract_y * self.data[y1][x2];
        value += fract_x * (1. - fract_y) * self.data[y2][x1];
        value += (1. - fract_x) * (1. - fract_y) * self.data[y2][x2];

        value
    }

    fn turbulence(&self, x: f64, y: f64, initial_size: f64) -> f64 {
        /* algorithm taken from https://lodev.org/cgtutor/randomnoise.html#Wood */
        let mut value = 0.0_f64;
        let mut size = initial_size;

        while size >= 1. {
            value += self.sample_smooth_noise(x / size, y / size) * size;
            size /= 2.0;
        }

        128.0 * value / initial_size
    }
}

pub fn rawwood(width: u32, height: u32, offsetstdev: f64) -> image::RgbImage {
    use rand::Rng;
    let mut imgbuf = image::RgbImage::new(width, height);

    let noise = Noise::gen_noise(width as usize, height as usize);

    /* algorithm taken and modified from https://lodev.org/cgtutor/randomnoise.html#Wood */
    let wavenumber = 0.0811; // dimension: # per px
    let turb = 14.6; //makes twists
    let turb_size = 32.0; //initial size of the turbulence

    let mut rng = rand::thread_rng();
    let distr = rand_distr::Normal::new(0., offsetstdev).unwrap();
    let offset_x = rng.sample(distr);
    let offset_y = rng.sample(distr);
    let phase = rng.sample(Uniform::from(0.0..std::f64::consts::PI));

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let x_value_times_scale = x as f64 - width as f64 / 2. + offset_x; // dimension: px
        let y_value_times_scale = y as f64 - height as f64 / 2. + offset_y; // dimension: px
        let dist_value_times_scale = x_value_times_scale.hypot(y_value_times_scale)
            + turb * noise.turbulence(x as f64, y as f64, turb_size) / 256.0;
        let sine_value = 88.0
            * ((wavenumber * dist_value_times_scale * std::f64::consts::PI + phase).sin())
                .abs()
                .powf(0.4);
        *pixel = image::Rgb([120 + sine_value as u8, 70 + sine_value as u8, 70]);
    }

    imgbuf
}
