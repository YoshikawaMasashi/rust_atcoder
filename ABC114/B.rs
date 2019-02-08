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

fn abs(x: i64) -> i64 {
    if x < 0 {
        -x
    } else{
        x
    }
}

fn main() {
    let s = read_line();
    let mut n: std::vec::Vec<i64> = std::vec::Vec::new();
    for c in s.chars() {
        n.push(c as i64 - 48);
    }
    let mut ret: i64 = 1000;

    for i in 0..n.len() - 2 {
        ret = std::cmp::min(ret, abs(753 - (100*n[i] + 10*n[i+1] + n[i+2])))
    }
    println!("{}", ret);
}