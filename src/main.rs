mod rustpbyractice;
mod hackerrank;
mod homework;
mod practice;
mod testhomework;

fn main() {
    const H: u32 = 13;
    const W: u32 = 13;
    let x_cent = W / 2;
    let y_cent = H / 2;

    for y in 0..H {
        for x in 0..W {
            let dist_x = (x as i32 - x_cent as i32).abs();
            let dist_y = (y as i32 - y_cent as i32).abs();

            let is_inside = dist_x + dist_y <= y_cent as i32;

            let sum = if is_inside { "*" } else { " " };
            print!("{}", sum);
        }
        println!();
    }
}





