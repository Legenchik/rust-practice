#[test]
fn main() {
    const H: u32 = 11;
    const W: u32 = 11;
    let x_cent = W/2;
    let y_cent = H/2;
    for y in 0..H {
        for x in 0..W {
            let centr =x==x_cent;
            let k = if y<=y_cent {x==(x_cent-y)} else {x==(y-x_cent)};
            let k2 = if y<=y_cent {x==(x_cent+y) } else {x==(W-1)-(y-x_cent)};
            let is = centr || k|| k2;
            //print!("y={}, x={}, xc={} ", y,x,x_cent);
            let sum = if is {
                "*"
            }else {
                " "
            };
            print!("{}", sum);
        }
        println!();
    }
}