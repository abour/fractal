mod color;

use image;
use rand::{thread_rng, Rng};
use std::sync::mpsc;
use std::thread;

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 1024;
const BUF_SIZE: u32 = WIDTH * HEIGHT * 3;
const NB_SAMPLES: u32 = 50;
const SIZE: f64 = 0.000000001;
const MAX_ITER: u32 = 1000;

fn main() {
    let px: f64 = -0.5557506;
    let py: f64 = -0.55560;
    let mut buf: Vec<u8> = vec![0; BUF_SIZE as usize];

    let (tx, rx) = mpsc::sync_channel(100);

    let chunk_size = HEIGHT / num_cpus::get() as u32;
    for offset in (0..HEIGHT).step_by(chunk_size as usize) {
        let tx = tx.clone();
        thread::spawn(move || {
            // let (line, line_number) = render_line(y, px, py);
            // tx.send((line, line_number)).await.unwrap();
            for y in offset..offset + chunk_size {
                let (line, line_number) = render_line(y, px, py);
                tx.send((line, line_number)).unwrap();
            }
        });
    }

    drop(tx);

    let mut finished: f64 = 0.;
     for res in rx.iter() {
        finished += 1.;
        let percentage_finished = (finished / (HEIGHT as f64)) * 100.;
        print!("Progress: {}%\r", percentage_finished as u32);

        let (line, line_number) = res;
        write_line(&mut buf, &line, line_number);
    }
    image::save_buffer("fractal.png", &buf, WIDTH, HEIGHT, image::ColorType::Rgb8).unwrap();
}

fn write_line(buf: &mut Vec<u8>, line: &Vec<u8>, line_number: u32) {
    for i in 0..WIDTH {
        buf[(((line_number * WIDTH) + i) * 3) as usize] = line[(i * 3) as usize];
        buf[((((line_number * WIDTH) + i) * 3) + 1) as usize] = line[((i * 3) + 1) as usize];
        buf[((((line_number * WIDTH) + i) * 3) + 2) as usize] = line[((i * 3) + 2) as usize];
    }
}

fn render_line(line_number: u32, px: f64, py: f64) -> (Vec<u8>, u32) {
    let mut rng = thread_rng();

    let line_size = WIDTH * 3;
    let mut line: Vec<u8> = vec![0; line_size as usize];

    for x in 0..WIDTH {
        let sampled_colours = (0..NB_SAMPLES)
            .map(|_| {
                let nx = SIZE * (((x as f64) + rng.gen_range(0., 1.0)) / (WIDTH as f64)) + px;
                let ny =
                    SIZE * (((line_number as f64) + rng.gen_range(0., 1.0)) / (HEIGHT as f64)) + py;
                let (m_res, m_iter) = mandelbrot_iter(nx, ny);
                paint(m_res, m_iter)
            })
            .map(|(r, g, b)| (r as i32, g as i32, b as i32));

        let (r, g, b): (i32, i32, i32) = sampled_colours
            .fold((0, 0, 0), |(cr, cg, cb), (r, g, b)| {
                (cr + r, cg + g, cb + b)
            });

        line[(x * 3) as usize] = ((r as f64) / (NB_SAMPLES as f64)) as u8;
        line[((x * 3) + 1) as usize] = ((g as f64) / (NB_SAMPLES as f64)) as u8;
        line[((x * 3) + 2) as usize] = ((b as f64) / (NB_SAMPLES as f64)) as u8;
    }

    return (line, line_number);
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
