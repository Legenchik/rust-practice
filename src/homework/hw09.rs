fn rotate(s: String, n: isize) -> String {
    let len = s.len() as isize;
    let shift = (n % len + len) % len; // Обробка негативних значень та значень, що перевищують довжину рядка
    let mut result = String::new();
    if shift == 0 {
        return s; // Якщо зсув дорівнює 0, повертаємо оригінальний рядок
    }
    for i in 0..s.len() {
        let new_index = (i as isize + len - shift) % len;
        result.push(s.chars().nth(new_index as usize).unwrap());
    }
    result
}
#[test]
fn test() {
    let s = "abcdefgh";
    let shifts = [
        (0,  "abcdefgh"),
        (8,  "abcdefgh"),
        (-8, "abcdefgh"),
        (1,  "habcdefg"),
        (2,  "ghabcdef"),
        (10, "ghabcdef"),
        (-1, "bcdefgha"),
        (-2, "cdefghab"),
        (-10,"cdefghab"),
    ];


    shifts
        .iter()
        .for_each(|(n, exp)|
            assert_eq!(
                rotate(s.parse().unwrap(), *n),
                exp.to_string()
            )
        );
}
