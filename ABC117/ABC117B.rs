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

fn main() {
    let n = str_to_ints(read_line())[0];
    let ls = str_to_ints(read_line());
    let mut max_l = 0;
    let mut sum_l = 0;
    for l in ls {
        if l > max_l {
            max_l = l;
        }
        sum_l += l;
    }
    if max_l < sum_l - max_l {
        println!("Yes");
    } else {
        println!("No");
    }
}
