extern crate image;

use rand::distributions::{Distribution, Uniform};

fn gen_noise(width: usize, height: usize) -> Vec<Vec<f64>> {
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
    return noise;
}

fn smooth_noise(noise: &Vec<Vec<f64>>, x: f64, y: f64, width: usize, height: usize) -> f64 {
    let fract_x = x.fract();
    let fract_y = y.fract();

    //wrap around
    let x1: usize = ((x as usize) + width) % width;
    let y1: usize = ((y as usize) + height) % height;

    //neighbor values
    let x2: usize = (x1 + width - 1) % width;
    let y2: usize = (y1 + height - 1) % height;

    //smooth the noise with bilinear interpolation
    let mut value = 0.0;
    value += fract_x * fract_y * noise[y1][x1];
    value += (1. - fract_x) * fract_y * noise[y1][x2];
    value += fract_x * (1. - fract_y) * noise[y2][x1];
    value += (1. - fract_x) * (1. - fract_y) * noise[y2][x2];

    return value;
}

fn turbulence(noise: &Vec<Vec<f64>>, x: f64, y: f64, initial_size: f64, width: u32, height: u32) -> f64 {
    /* algorithm taken from https://lodev.org/cgtutor/randomnoise.html#Wood */
    let mut value = 0.0f64;
    let mut size = initial_size;

    while size >= 1. {
        value += smooth_noise(
            noise,
            x / size,
            y / size,
            width as usize,
            height as usize,
        ) * size;
        size /= 2.0;
    }

    return 128.0 * value / initial_size;
}

fn rawwood(width: u32, height: u32) -> image::RgbImage {
    let mut imgbuf = image::RgbImage::new(width, height);

    let noise = gen_noise(width as usize, height as usize);

    /* algorithm taken from https://lodev.org/cgtutor/randomnoise.html#Wood */
    let xy_period = 12.0; //number of rings
    let turb_power = 0.15; //makes twists
    let turb_size = 32.0; //initial size of the turbulence

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let x_value = (x as f64 - width as f64 / 2.) / (width as f64);
        let y_value = (y as f64 - height as f64 / 2.) / (height as f64);
        let dist_value = (x_value * x_value + y_value * y_value).sqrt()
            + turb_power * turbulence(&noise, x as f64, y as f64, turb_size, width, height) / 256.0;
        let sine_value = 128.0 * ((2. * xy_period * dist_value * 3.14159).sin()).abs();
        *pixel = image::Rgb([120 + sine_value as u8, 70 + sine_value as u8, 70]);
    }

    return imgbuf;
}

fn rawboard(square_size_in_pixel: f32) -> image::RgbImage {
    /* Numbers based on physical measurements */
    let tak1_color = image::Rgb([193, 193, 193]);
    let tam2hue_color = image::Rgb([204, 136, 82]);
    let tam2zo1_color = image::Rgb([32, 72, 38]);
    let tam2nua2_color = image::Rgb([98, 133, 177]);
    let line_width = 0.04;
    let line_color = image::Rgb([10, 10, 10]);
    let cwidth = 6.376 * 2.;
    let cheight = 9.642 * 2.;

    let imgx = (square_size_in_pixel * cwidth) as u32;
    let imgy = (square_size_in_pixel * cheight) as u32;

    /* first draw the board */
    let mut imgbuf = image::ImageBuffer::from_pixel(imgx, imgy, tak1_color);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        /* the size of each square is 1.0 */
        /* the center of the image is the origin */
        let cx = x as f32 / square_size_in_pixel - cwidth / 2.;
        let cy = y as f32 / square_size_in_pixel - cheight / 2.;

        /* the board is already drawn */
        /* then draw the squares */
        if -1.5 <= cx && cx <= 1.5 && -1.5 <= cy && cy <= 1.5 {
            *pixel = tam2hue_color;
        }
        if 1.5 <= cx.abs() && cx.abs() <= 2.5 && 1.5 <= cy.abs() && cy.abs() <= 2.5 {
            *pixel = tam2hue_color;
        }
        if (-2.5 <= cx && cx <= 2.5 && -0.5 <= cy && cy <= 0.5)
            || (-2.5 <= cy && cy <= 2.5 && -0.5 <= cx && cx <= 0.5)
        {
            *pixel = if -0.5 <= cx && cx <= 0.5 && -0.5 <= cy && cy <= 0.5 {
                tam2zo1_color
            } else {
                tam2nua2_color
            }
        }

        /* Now draw the lines */

        /* horizontal and vertical */
        for loc in vec![-4.5, -3.5, -2.5, -1.5, -0.5, 0.5, 1.5, 2.5, 3.5, 4.5] {
            if (loc - cx).abs() <= line_width / 2.0 && cy.abs() <= 4.5 + line_width / 2.0 {
                *pixel = line_color;
            }
            if (loc - cy).abs() <= line_width / 2.0 && cx.abs() <= 4.5 + line_width / 2.0 {
                *pixel = line_color;
            }
        }

        /* tam2nua2 corners */
        if (2.5 - line_width * 2. - cx.abs()).abs() <= line_width / 2.0
            && cy.abs() <= 0.5 - line_width * 1.5
            && 0.25 <= cy.abs()
        {
            *pixel = line_color;
        }
        if (2.5 - line_width * 2. - cy.abs()).abs() <= line_width / 2.0
            && cx.abs() <= 0.5 - line_width * 1.5
            && 0.25 <= cx.abs()
        {
            *pixel = line_color;
        }
        if (0.5 - line_width * 2. - cy.abs()).abs() <= line_width / 2.0
            && cx.abs() <= 2.5 - line_width * 1.5
            && 2.25 <= cx.abs()
        {
            *pixel = line_color;
        }
        if (0.5 - line_width * 2. - cx.abs()).abs() <= line_width / 2.0
            && cy.abs() <= 2.5 - line_width * 1.5
            && 2.25 <= cy.abs()
        {
            *pixel = line_color;
        }

        /* tam2hue diagonal */
        if (cx + cy).abs() <= line_width * std::f32::consts::FRAC_1_SQRT_2 && cx.abs() <= 2.5 {
            *pixel = line_color;
        }
        if (cx - cy).abs() <= line_width * std::f32::consts::FRAC_1_SQRT_2 && cx.abs() <= 2.5 {
            *pixel = line_color;
        }
    }

    return imgbuf;
}

fn main() -> Result<(), rand_distr::NormalError> {
    let rawboard = rawboard(100.0);
    rawboard.save("fractal.png").unwrap();

    let rawwood = rawwood(80, 80);
    rawwood.save("rawwood.png").unwrap();

    // If I succeed in implementing GIMP's bump_map later, then I will resurrect this code
    /*
    extern crate cloth_bumpmap;
    [dependencies]
    cloth_bumpmap = "0.1.1"
    let (width, height) = rawboard.dimensions();
    let bumpmap = cloth_bumpmap::cloth_bumpmap(width, height)?;

    bumpmap.save("bumpmap.png").unwrap();

    let clothed = emboss::emboss::apply_bump_map(
        rawboard,
        bumpmap,
        std::f64::consts::PI / 4.0 * 3.0,
        std::f64::consts::PI / 4.0,
    )
    .unwrap();
    clothed.save("clothed.png").unwrap();
    */
    Ok(())
}
