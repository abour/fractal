mod color;

use rand::{thread_rng, Rng};
use rayon::prelude::*;

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 1024;
const BUF_SIZE: u32 = WIDTH * HEIGHT * 3;
const NB_SAMPLES: u32 = 50;
const SIZE: f64 = 0.000000001;
const MAX_ITER: u32 = 1000;

fn main() {
    let px: f64 = -0.5557506;
    let py: f64 = -0.55560;
    let mut buf = vec![0; BUF_SIZE as usize];

    buf.par_chunks_mut(3 * WIDTH as usize)
       .enumerate()
       .for_each(|(y, line)| render_line(line, y as u32, px, py));
    image::save_buffer("fractal.png", &buf, WIDTH, HEIGHT, image::ColorType::Rgb8).unwrap();
}

fn render_line(line: &mut [u8], y: u32, px: f64, py: f64) {
    let mut rng = thread_rng();

    for x in 0..WIDTH {
        let (r, g, b) = (0..NB_SAMPLES)
            .map(|_| {
                let nx = SIZE * (((x as f64) + rng.gen_range(0., 1.0)) / (WIDTH as f64)) + px;
                let ny = SIZE * (((y as f64) + rng.gen_range(0., 1.0)) / (HEIGHT as f64)) + py;
                let (m_res, m_iter) = mandelbrot_iter(nx, ny);
                paint(m_res, m_iter)
            })
            .map(|(r, g, b)| (r as f64, g as f64, b as f64))
            .fold((0., 0., 0.), |(cr, cg, cb), (r, g, b)| {
                (cr + r, cg + g, cb + b)
            });

        line[(x * 3) as usize] = (r / (NB_SAMPLES as f64)) as u8;
        line[((x * 3) + 1) as usize] = (g / (NB_SAMPLES as f64)) as u8;
        line[((x * 3) + 2) as usize] = (b / (NB_SAMPLES as f64)) as u8;
    }
}

fn paint(r: f64, n: u32) -> (u8, u8, u8) {
    if r > 4. {
        return color::hsl_to_rgb(n as f64 / 800. * r, 1., 0.5);
    } else {
        return (255, 255, 255);
    }
}

fn mandelbrot_iter(px: f64, py: f64) -> (f64, u32) {
    let (mut x, mut y, mut xx, mut yy) = (0., 0., 0., 0.);
    let mut xy;

    for i in 0..MAX_ITER {
        xx = x * x;
        yy = y * y;
        xy = x * y;
        if xx + yy > 4. {
            return (xx + yy, i);
        }
        x = xx - yy + px;
        y = 2. * xy + py;
    }

    return (xx + yy, MAX_ITER);
}
