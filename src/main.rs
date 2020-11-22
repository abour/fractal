mod color;

use image;
use rand::{thread_rng, Rng};
use tokio::sync::mpsc;

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 1024;
const BUF_SIZE: u32 = WIDTH * HEIGHT * 3;
const NB_SAMPLES: u32 = 50;
const SIZE: f64 = 0.000000001;
const MAX_ITER: u32 = 1000;

#[tokio::main]
async fn main() {
    let blocking_task = tokio::spawn(async {
        let px: f64 = -0.5557506;
        let py: f64 = -0.55560;
        let mut buf: Vec<u8> = Vec::with_capacity(BUF_SIZE as usize);
        buf.resize(BUF_SIZE as usize, 0);

        let (tx, mut rx) = mpsc::channel(100);

        for y in 0..HEIGHT {
            let tx = tx.clone();
            tokio::spawn(async move {
                let (line, line_number) = render_line(y, px, py);
                tx.send((line, line_number)).await.unwrap();
            });
        }

        drop(tx);

        let mut finished: f64 = 0.;
        while let Some(res) = rx.recv().await {
            finished += 1.;
            let percentage_finished = (finished / (HEIGHT as f64)) * 100.;
            print!("Progress: {}%\r", percentage_finished as u32);

            let (line, line_number) = res;
            write_line(&mut buf, &line, line_number);
        }
        image::save_buffer("fractal.png", &buf, WIDTH, HEIGHT, image::ColorType::Rgb8).unwrap();
    });

    blocking_task.await.unwrap();
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
    let mut line: Vec<u8> = Vec::with_capacity(line_size as usize);
    line.resize(line_size as usize, 0);

    for x in 0..WIDTH {
        let sampled_size = NB_SAMPLES * 3;
        let mut sampled_colours: Vec<u8> = Vec::with_capacity(sampled_size as usize);
        sampled_colours.resize(sampled_size as usize, 0);

        for i in 0..NB_SAMPLES {
            let nx = SIZE * (((x as f64) + rng.gen_range(0., 1.0)) / (WIDTH as f64)) + px;
            let ny =
                SIZE * (((line_number as f64) + rng.gen_range(0., 1.0)) / (HEIGHT as f64)) + py;
            let (m_res, m_iter) = mandelbrot_iter(nx, ny);
            let (paint_r, paint_g, paint_b) = paint(m_res, m_iter);

            sampled_colours[(i * 3) as usize] = paint_r;
            sampled_colours[((i * 3) + 1) as usize] = paint_g;
            sampled_colours[((i * 3) + 2) as usize] = paint_b;
        }
        let mut r: i32 = 0;
        let mut g: i32 = 0;
        let mut b: i32 = 0;
        for i in 0..NB_SAMPLES {
            r += (sampled_colours[(i * 3) as usize]) as i32;
            g += (sampled_colours[((i * 3) + 1) as usize]) as i32;
            b += (sampled_colours[((i * 3) + 2) as usize]) as i32;
        }

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
