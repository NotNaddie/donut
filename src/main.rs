use {std::f64::consts::PI, clap::Parser};

const CHARS: [&str; 12] = [".", ",", "-", "~", ":", ";", "=", "!", "*", "#", "$", "@"];

const W: usize = 58; // Display Width
const H: usize = 29; // Display Height

/*  The width has to be at least H*2 due to the chars in terminal being
    taller, their ratio is aprox. 2:1 H:W */

// Arc radius
const R1: f64 = 1.0;
const R2: f64 = 2.0;
// Main radius

// Distance from camera
const KZ: f64 = 5.0;

const TS: f64 = 0.02; // theta spacing (theta as t)
const PS: f64 = 0.05; // phi spacing (phi as p)

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    
    #[arg(short, long)]
    xspeed: Option<f64>,

    #[arg(short, long)]
    yspeed: Option<f64>,
    
    #[arg(short, long)]
    padding: Option<usize>,
    
    #[arg(short, long)]
    size: Option<usize>,
    
    /* #[arg(short, long, default_value_t = false)]
    rotate: bool, */
}

/*
    * Calculate each point, its lighting, plot them and return a buffer representing a frame of the animation according to a and b,
    angles in radians provided as arguments
*/
fn get_buffer<'s>(alpha: f64, beta: f64, w: usize, h: usize) -> Vec<Vec<&'s str>> {
    let (mut buffer, mut z_buffer): (Vec<Vec<&str>>, Vec<Vec<f64>>) = (vec![vec![" "; w]; h], vec![vec![0_f64; w]; h]);
    
    let (sina, sinb): (f64, f64) = (alpha.sin(), beta.sin()); // Precomputed sines and cosines
    let (cosa, cosb): (f64, f64) = (alpha.cos(), beta.cos());
    
    let mut t: f64 = 0.0; // Theta value
    let cam_const = KZ*(3_f64*(h as f64)/4_f64)/(R1+R2); // So the donut doesn't exceed the window's size
    
    while t < 2.0*PI {
        let (sint, cost): (f64, f64) = (t.sin(), t.cos()); // Precomputed sine and cosine of theta
        
        let mut p: f64 = 0.0; // Phi value
        
        while p < 2.0*PI {
            let (sinp, cosp): (f64, f64) = (p.sin(), p.cos()); // Precompute...
            
            // Plotting points in XY plane
            let (cx, cy): (f64, f64) = (R2 + R1*cosp, R1*sinp); 
            
            // Rotating points in XYZ plane according to 3D rotation matrices
            let (x, y, z): (f64, f64, f64) = (
                cosb*cost*cx - sinb*(cosa*cy + sina*sint*cx),
                sinb*cost*cx + cosb*(cosa*cy + sina*sint*cx),
                sina*cy - cosa*sint*cx + KZ
            );
            
            // Precompute one over z
            let ooz = 1.0_f64/z;
            
            /* 
                Projected values from similar triangles calculation
                H gets divided by 2 because of the char height
            */
            let (px, py): (i32, i32) = (
                ((w as f64)/2.0 + ooz*cam_const*x) as i32,
                ((h as f64) - ooz*cam_const*y) as i32 / 2
            );
            
            // Lighting caclulation, dot product between vector (0, 1, -1) * Normal of point
            let l: f64 = cosp*(sinb*cost + cosa*sint) + cosb*(cosa*sinp + sina*sint*cosp) - sina*sinp;
            // Ranges [ -sqrt(2), sqrt(2) ]
            
            // Plotting only if the value is higher than the one in the z_buffer (initialized as 0)
            if ooz > z_buffer[py as usize][px as usize] {
                // Save z_buffer position
                z_buffer[py as usize][px as usize] = ooz;
                // Plot point as char at l*8 (normalized to range between 0 and 11)
                buffer[py as usize][px as usize] = CHARS[(l*8_f64)as usize];
            }
            
            p += PS; // increment by phi spacing
        }
        
        t += TS; // increment by theta spacing
    }
    
    buffer
}

/*
    * Print out a frame according to a buffer
*/
fn print_buffer<'s>(output_buffer: Vec<Vec<&'s str>>) {
    use std::io::{self, Write};
    
    print!("\x1b[H"); // Print from home position
    print!("\x1b[J"); // Clear terminal
    
    for line in output_buffer { // Loop through every line
        let mut line_text = String::from(""); // Create line text
        
        for pixel in line { // Loop through chars and add them to line text
            line_text += pixel;
        }
        
        println!("{}", line_text); // Print out line text
    }
    
    io::stdout().flush().unwrap();
}

fn main() {
    
    // Set speeds for a & b increments
    
    let cli = Cli::parse();

    let (b_speed, a_speed): (f64, f64) =
        (
            if let Some(xspeed) = cli.xspeed {
                xspeed
            } else { 1_f64 },
            if let Some(yspeed) = cli.yspeed {
                yspeed
            } else { 1_f64 }
        );

    let (width, height): (usize, usize) =
        if let (Some(p), Some(s)) = (cli.padding, cli.size) {
            ((p*2_usize + s*2_usize), s)
        } else if let (Some(s), None) = (cli.size, cli.padding) {
            (s*2_usize, s)
        } else { (W, H) };

    let (mut a, mut b): (f64, f64) = (0.0, 0.0);
    let (a_rate, b_rate): (f64, f64) = (a_speed / 100.0, b_speed / 100.0);
    
    // Infinite loop
    loop {
        print_buffer(get_buffer(a, b, width, height)); // Get and print frame

        a += a_rate; // Increment angles
        b += b_rate;
        
        if a > 2.0*PI { a = 0.0; } // If they exceed 2*PI, reset to 0
        if b > 2.0*PI { b = 0.0; }
    }
}
