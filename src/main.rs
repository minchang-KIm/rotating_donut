use std::{thread, time};

fn main() {
    // ANSI escape sequence to clear the console
    print!("\x1b[2J");

    let mut A:f32 = 0.0;
    let mut B:f32 = 0.0;
    let mut b = [' ' as u8; 1760];
    let mut z = [0.0; 1760];

    loop {
        // Clearing the buffers
        b.iter_mut().for_each(|x| *x = ' ' as u8);
        z.iter_mut().for_each(|x| *x = 0.0);

        // Looping through angles
        (0..628).map(|j| j as f32 / 100.0).for_each(|j| {
            (0..628).map(|i| i as f32 / 100.0).for_each(|i| {
                let sini = i.sin();
                let cosj = j.cos();
                let sinA = A.sin();
                let sinj = j.sin();
                let cosA = A.cos();
                let cosj2 = cosj + 2.0;
                let mess = 1.0 / (sini * cosj2 * sinA + sinj * cosA + 5.0);
                let cosi = i.cos();
                let cosB = B.cos();
                let sinB = B.sin();
                let t = sini * cosj2 * cosA - sinj * sinA;

                let x = (40.0 + 30.0 * mess * (cosi * cosj2 * cosB - t * sinB)) as usize;
                let y = (12.0 + 15.0 * mess * (cosi * cosj2 * sinB + t * cosB)) as usize;
                let o = x + 80 * y;
                let N = 8 * ((sinj * sinA - sini * cosj * cosA) * cosB - sini * cosj * sinA - sinj * cosA - cosi * cosj * sinB) as usize;

                if y > 0 && y < 22 && x > 0 && x < 80 && mess > z[o] {
                    z[o] = mess;
                    b[o] = match N.min(70) {
                        0 => '.',
                        1 => ',',
                        2 => '-',
                        3 => '~',
                        4 => ':',
                        5 => ';',
                        6 => '=',
                        7 => '!',
                        8 => '*',
                        9 => '#',
                        10 => '$',
                        11 => '@',
                        _ => unreachable!(), // Handles cases where N exceeds 70
                    } as u8;
                }
            });
        });
        // Printing the buffer
        for (i, &c) in b.iter().enumerate() {
            print!("{}", c as char);
            if (i + 1) % 80 == 0 {
                println!();
            }
        }

        // Sleeping for a short while to control animation speed
        thread::sleep(time::Duration::from_millis(50));

        // Updating angles for next iteration
        A += 0.08;
        B += 0.02;
    }
}