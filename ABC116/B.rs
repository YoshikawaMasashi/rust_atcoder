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

fn f(n: i64) -> i64 {
    if n % 2 == 0 {
        n / 2
    } else {
        3 * n + 1
    }
}

fn main() {
    let s = read_ints()[0];
    let mut set_a: std::collections::HashSet<i64> = std::collections::HashSet::new();

    let mut a: i64 = s;
    for i in 0..1000000 {
        if set_a.contains(&a) {
            println!("{}", i + 1);
            break;
        }
        set_a.insert(a);
        a = f(a);
    }
}