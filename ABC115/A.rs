fn read_line() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim_right().to_owned()
}

fn str_to_ints(s: String) -> std::vec::Vec<i64> {
    let mut ret = std::vec::Vec::new();
    let ws = s.split_whitespace();
    for w in ws {
        ret.push(w.parse().unwrap());
    }
    ret
}

fn read_ints() -> std::vec::Vec<i64> {
    str_to_ints(read_line())
}

fn main() {
    let d: i64 = read_ints()[0];
    if d == 25 {
        println!("Christmas");
    } else if d == 24 {
        println!("Christmas Eve");
    } else if d == 23 {
        println!("Christmas Eve Eve");
    } else if d == 22 {
        println!("Christmas Eve Eve Eve");
    }
}