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

fn bp_num(n: i64) -> i64 {
    if n == 0 {
        return 1;
    } else {
        return 2 * bp_num(n - 1) + 3;
    }
}

fn p_num(n: i64) -> i64 {
    if n == 0 {
        return 1;
    } else {
        return 2 * p_num(n - 1) + 1;
    }
}

fn dfs(n: i64, x : i64) -> i64 {
    if n == 0 {
        return 1;
    }

    let bp_num_nm1 = bp_num(n - 1);
    let p_num_nm1 = p_num(n - 1);

    if x == 0 {
        return 0;
    } else if x == bp_num(n) - 1 {
        return p_num(n);
    } else if x == 1 + bp_num_nm1 {
        return p_num_nm1 + 1;
    } else if x > 1 + bp_num_nm1 {
        return p_num_nm1 + 1 + dfs(n - 1, x - bp_num_nm1 - 2);
    } else {
        return dfs(n - 1, x - 1);
    }
}
fn main() {
    let nx = read_ints();
    let n = nx[0];
    let x = nx[1];

    println!("{}", dfs(n, x - 1));
}