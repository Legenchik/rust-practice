use std::collections::HashSet;

fn solve_rebus() {
    let mut solve=0;
    for m in 1..=8 {
        for u in 1..=8 {
            for h in 1..=8 {
                for a in 1..=8 {
                    for s in 1..=8 {
                        for l in 1..=8 {
                            for o in 1..=8 {
                                for n in 1..=8 {
                                    let mut digits = HashSet::new();
                                    digits.insert(m);
                                    digits.insert(u);
                                    digits.insert(h);
                                    digits.insert(a);
                                    digits.insert(s);
                                    digits.insert(l);
                                    digits.insert(o);
                                    digits.insert(n);

                                    if digits.len() == 8 {
                                        let muha = m * 1000 + u * 100 + h * 10 + a;
                                        let slon = s * 1000 + l * 100 + o * 10 + n;

                                        if muha * a == slon {
                                            println!("Розв'язок знайдено:");
                                            println!("  {}{}{}{}", format!("{:4}","M"),format!("{:4}","U"),format!("{:4}","H"),format!("{:4}","A"));
                                            println!("  {}{}{}{}", format!("{:1}",m),format!("{:4}",u),format!("{:4}",h),format!("{:4}",a));
                                            println!("*");
                                            println!("              {}", a);
                                            println!("----------------");
                                            println!("  {}{}{}{}", format!("{:4}","S"),format!("{:4}","L"),format!("{:4}","O"),format!("{:4}","N"));
                                            println!("  {}{}{}{}", format!("{:1}",s),format!("{:4}",l),format!("{:4}",o),format!("{:4}",n));
                                            println!();
                                            solve = solve + 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if solve!=0{
        println!("Кількість розв'язків: {}",solve);
    }else {
        println!("Розв'язок не знайдено.");
    }

}
#[test]
fn main() {
    solve_rebus();
}