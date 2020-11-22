extern crate image;
pub mod color;

use rand::{thread_rng, Rng};

fn main() {
    let width: u32 = 512;
    let height: u32 = 512;
    let size = width * height * 3;

    let mut buf: Vec<u8> = Vec::with_capacity(size as usize);
    buf.resize(size as usize, 0);

    render(
        &mut buf,
        width,
        height,
        50,
        -0.5557506,
        -0.55560,
        0.000000001,
        1000,
    );

    image::save_buffer("image.png", &buf, width, height, image::ColorType::Rgb8).unwrap();
}

fn render(
    buf: &mut Vec<u8>,
    width: u32,
    height: u32,
    nb_samples: i32,
    px: f64,
    py: f64,
    size: f64,
    max_iter: i32,
) {
    let mut rng = thread_rng();

    for y in 0..height {
        for x in 0..width {
            let sampled_size = nb_samples * 3;
            let mut sampled_colours: Vec<u8> = Vec::with_capacity(sampled_size as usize);
            sampled_colours.resize(sampled_size as usize, 0);

            for i in 0..nb_samples {
                let nx = size * (((x as f64) + rng.gen_range(0., 1.0)) / (width as f64)) + px;
                let ny = size * (((y as f64) + rng.gen_range(0., 1.0)) / (height as f64)) + py;
                let (m_res, m_iter) = mandelbrot_iter(nx, ny, max_iter);
                let (paint_r, paint_g, paint_b) = paint(m_res, m_iter);

                sampled_colours[(i * 3) as usize] = paint_r;
                sampled_colours[((i * 3) + 1) as usize] = paint_g;
                sampled_colours[((i * 3) + 2) as usize] = paint_b;
            }
            let mut r: i32 = 0;
            let mut g: i32 = 0;
            let mut b: i32 = 0;
            for i in 0..nb_samples {
                r += (sampled_colours[(i * 3) as usize]) as i32;
                g += (sampled_colours[((i * 3) + 1) as usize]) as i32;
                b += (sampled_colours[((i * 3) + 2) as usize]) as i32;
            }
            buf[((y * width + x) * 3) as usize] = ((r as f64) / (nb_samples as f64)) as u8;
            buf[(((y * width + x) * 3) + 1) as usize] = ((g as f64) / (nb_samples as f64)) as u8;
            buf[(((y * width + x) * 3) + 2) as usize] = ((b as f64) / (nb_samples as f64)) as u8;
        }
    }
}

fn paint(r: f64, n: i32) -> (u8, u8, u8) {
    if r > 4. {
        return color::hsl_to_rgb((n as f64) / 800. * r, 1., 0.5);
    } else {
        return (255, 255, 255);
    }
}

fn mandelbrot_iter(px: f64, py: f64, max_iter: i32) -> (f64, i32) {
    let (mut x, mut y, mut xx, mut yy, mut xy) = (0., 0., 0., 0., 0.);

    for i in 0..max_iter {
        xx = x * x;
        yy = y * y;
        xy = x * y;
        if xx + yy > 4. {
            return (xx + yy, i);
        }
        x = xx - yy + px;
        y = 2. * xy + py;
    }

    return (xx + yy, max_iter);
}