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
    let n = read_ints()[0];
    let mut p: std::vec::Vec<i64> = std::vec::Vec::new();
    for i in 0..n {
        p.push(read_ints()[0]);
    }

    let mut sum_p: i64 = 0;
    let mut max_p: i64 = 0;
    for i in 0..n {
        sum_p += p[i as usize];
        max_p = std::cmp::max(max_p, p[i as usize]);
    }
    let ret: i64 = sum_p - max_p / 2;

    println!("{}", ret);
}