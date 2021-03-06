const TAK1: &[u8] = include_bytes!("optimum.png");

pub fn gen_bg(square_size_in_pixel: f32) -> image::RgbImage {
    if (square_size_in_pixel - 100.0).abs() < std::f32::EPSILON {
        return image::load_from_memory(TAK1).unwrap().to_rgb8();
    }

    rawboard(square_size_in_pixel)
    // If I succeed in implementing GIMP's bump_map later, then I will resurrect this code
    /*
    extern crate cloth_bumpmap;
    [dependencies]
    cloth_bumpmap = "0.1.1"
    let raw_board = rawboard(square_size_in_pixel);
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
    raw_board
    */
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

    let img_x = (square_size_in_pixel * cwidth) as u32;
    let img_y = (square_size_in_pixel * cheight) as u32;

    /* first draw the board */
    let mut imgbuf = image::ImageBuffer::from_pixel(img_x, img_y, tak1_color);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        /* the size of each square is 1.0 */
        /* the center of the image is the origin */
        let cx = x as f32 / square_size_in_pixel - cwidth / 2.;
        let cy = y as f32 / square_size_in_pixel - cheight / 2.;

        /* the board is already drawn */
        /* then draw the squares */
        if (-1.5..=1.5).contains(&cx) && (-1.5..=1.5).contains(&cy) {
            *pixel = tam2hue_color;
        }
        if (1.5..=2.5).contains(&cx.abs()) && (1.5..=2.5).contains(&cy.abs()) {
            *pixel = tam2hue_color;
        }
        if ((-2.5..=2.5).contains(&cx) && (-0.5..=0.5).contains(&cy))
            || ((-2.5..=2.5).contains(&cy) && (-0.5..=0.5).contains(&cx))
        {
            *pixel = if (-0.5..=0.5).contains(&cx) && (-0.5..=0.5).contains(&cy) {
                tam2zo1_color
            } else {
                tam2nua2_color
            }
        }

        /* Now draw the lines */

        /* horizontal and vertical */
        for loc in &[-4.5, -3.5, -2.5, -1.5, -0.5, 0.5, 1.5, 2.5, 3.5, 4.5] {
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

    imgbuf
}
