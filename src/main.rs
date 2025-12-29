use std::f64::consts::PI;

const CHARS: [&str; 12] = [".", ",", "-", "~", ":", ";", "=", "!", "*", "#", "$", "@"];

const W: f64 = 156.0;
const H: f64 = 46.0;

const R1: f64 = 1.0;
const R2: f64 = 2.0;

const K1: f64 = 10.0;
const K2: f64 = K1*H/12.0*(R1 + R2);

const TS: f64 = 0.02;
const PS: f64 = 0.03;

fn get_buffer<'s>(alpha: f64, beta: f64) -> [[&'s str; W as usize]; H as usize] {
    let mut buffer: [[&str; W as usize]; H as usize] = [[" "; W as usize]; H as usize];
    let mut y_buffer: [[f64; W as usize]; H as usize] = [[0.0; W as usize]; H as usize];

    let (sina, sinb): (f64, f64) = (alpha.sin(), beta.sin());
    let (cosa, cosb): (f64, f64) = (alpha.cos(), beta.cos());

    let mut t: f64 = 0.0;

    while t < 2.0*PI {
        let (sint, cost): (f64, f64) = (t.sin(), t.cos());

        let mut p: f64 = 0.0;

        while p < 2.0*PI {
            let (sinp, cosp): (f64, f64) = (p.sin(), p.cos());

            let (cx, cy): (f64, f64) = (R2 + R1*cosp, R1*sinp);
            let (x, y, z): (f64, f64, f64) = (
                cosa*cost*cx - sina*cy,
                cosb*(sina*cost*cx + cosa*cy) + sinb*sint*cx,
                cosb*sint*cx - sinb*(sina*cost*cx + cosa*cy)
            );

            let (px, py): (i32, i32) = (
                (W/2.0 + (K2*x/(K1+y))) as i32,
                (H + (K2*z/(K1+y))) as i32 / 2 // División tras conversión por generalización
            );

            let ooy = 1.0_f64/(y + K2);

            let l: f64 = cosa*cost*cosp-sina*sint + (cosb*(sina*cost*cosp + cosa*sint) - sinb*cost*sinp);

            if ooy > y_buffer[py as usize][px as usize] {
                y_buffer[py as usize][px as usize] = ooy;
                buffer[py as usize][px as usize] = CHARS[(l*8_f64)as usize];
            }
            
            p += PS;
        }

        t += TS;
    }

    buffer
}

fn print_buffer<'s>(output_buffer: [[&'s str; W as usize]; H as usize]) {
    for line in output_buffer {
        let mut line_text = String::from("");

        for pixel in line {
            line_text += pixel;
        }

        println!("{}", line_text);
    }
}

fn main() {
    use std::{thread, time};

    let (mut a, mut b): (f64, f64) = (0.0, 0.0);
    let (a_speed, b_speed): (f64, f64) = (2.0, 2.0);
    let (a_rate, b_rate): (f64, f64) = (a_speed / 50.0, b_speed / 50.0);
    
    loop {
        print_buffer(get_buffer(a, b));

        a += a_rate;
        b += b_rate;
        
        if a > 2.0*PI { a = 0.0; }
        if b > 2.0*PI { b = 0.0; }

        thread::sleep(time::Duration::from_millis(10));
    }
}