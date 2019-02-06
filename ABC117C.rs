fn read_line() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim_right().to_owned()
}

fn str_to_ints(s: String) -> std::vec::Vec<i32> {
    let mut ret = std::vec::Vec::new();
    let ws = s.split_whitespace();
    for w in ws {
        ret.push(w.parse().unwrap());
    }
    ret
}

fn read_ints() -> std::vec::Vec<i32> {
    str_to_ints(read_line())
}

fn main() {
    let nm = read_ints();
    let n = nm[0];
    let m = nm[1];
    let xs = read_ints();

    let mut xs_sorted: std::vec::Vec<i32> = xs.to_vec();
    xs_sorted.sort();

    let mut xs_diff: std::vec::Vec<i32> = std::vec::Vec::new();
    for i in 0..xs_sorted.len()-1 {
        xs_diff.push(xs_sorted[i+1] - xs_sorted[i]);
    }

    let mut xs_diff_sorted: std::vec::Vec<i32> = xs_diff.to_vec();
    xs_diff_sorted.sort();

    let mut ret = 0;
    if m - n > 0 {
        for i in 0..(m - n) as usize {
        ret += xs_diff_sorted[i];
        }
    }

    println!("{}", ret);
}