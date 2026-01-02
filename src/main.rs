use std::f64::consts::PI;

const CHARS: [&str; 12] = [".", ",", "-", "~", ":", ";", "=", "!", "*", "#", "$", "@"];

const W: f64 = 140.0; // Display Width
const H: f64 = 60.0; // Display Height

/*  The width has to be at least H*2 due to the chars in terminal being
    taller, their ratio is aprox. 2:1 H:W */

// Arc radius
const R1: f64 = 1.0;
const R2: f64 = 2.0;
// Main radius

// Distance from camera
const KZ: f64 = 5.0;
const K1: f64 = KZ*(3.0_f64*H/4.0_f64)/(R1 + R2);
// The donut's size will always be 3 fourths of the display's height

const TS: f64 = 0.02; // theta spacing (theta as t)
const PS: f64 = 0.05; // phi spacing (phi as p)

/*
    * Calculate each point, its lighting, plot them and return a buffer representing a frame of the animation according to a and b,
    angles in radians provided as arguments.
*/
fn get_buffer<'s>(alpha: f64, beta: f64) -> [[&'s str; W as usize]; H as usize] {
    let mut buffer: [[&str; W as usize]; H as usize] = [[" "; W as usize]; H as usize];
    let mut z_buffer: [[f64; W as usize]; H as usize] = [[0.0; W as usize]; H as usize];

    let (sina, sinb): (f64, f64) = (alpha.sin(), beta.sin()); // Precomputed sines and cosines
    let (cosa, cosb): (f64, f64) = (alpha.cos(), beta.cos());

    let mut t: f64 = 0.0; // Theta value

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
                (W/2.0 + ooz*K1*x) as i32,
                (H - ooz*K1*y) as i32 / 2
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
fn print_buffer<'s>(output_buffer: [[&'s str; W as usize]; H as usize]) {
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
    let (mut a, mut b): (f64, f64) = (0.0, 0.0);
    let (a_speed, b_speed): (f64, f64) = (0.5, 5.0);
    let (a_rate, b_rate): (f64, f64) = (a_speed / 50.0, b_speed / 50.0);

    // Infinite loop
    loop {
        print_buffer(get_buffer(a, b)); // Get and print frame

        a += a_rate; // Increment angles
        b += b_rate;
        
        if a > 2.0*PI { a = 0.0; } // If they exceed 2*PI, reset to 0
        if b > 2.0*PI { b = 0.0; }
    }
}