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
    let s = read_line();
    let mut s_vec: std::vec::Vec<i64> = std::vec::Vec::new();

    for c in s.chars() {
        if c == '1' {
            s_vec.push(1);
        } else {
            s_vec.push(0);
        }
    }

    if s_vec[0] != 1 || s_vec[s.len() - 1] != 0 {
        println!("-1");
        return
    }
    for i in 0..(s.len() / 2) {
        if s_vec[i] != s_vec[s.len() - i - 2] {
            println!("-1");
            return
        }
    }
    let mut s_vec_prime: std::vec::Vec<i64> = std::vec::Vec::new();
    for i in 0..s_vec.len() {
        if i == s_vec.len() - 1 {
            s_vec_prime.push(1);
        } else {
            s_vec_prime.push(s_vec[i]);
        }
    }

    let mut parents: std::vec::Vec<i64> = std::vec::Vec::new();
    let mut tmp_parent: i64 = s.len() as i64;
    for i in (0..(s.len() - 1)).rev() {
        if s_vec_prime[i + 1] == 1 {
            tmp_parent = i as i64 + 2;
        }
        parents.insert(0, tmp_parent);
    }

    for i in 0..(s.len() - 1) {
        println!("{} {}", i+1, parents[i]);
    }
}