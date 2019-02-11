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
    let abbcca = read_ints();
    let ab = abbcca[0];
    let bc = abbcca[1];
    let ca = abbcca[2];

    if ab > bc && ab > ca {
        println!("{}", bc * ca / 2);
    }
    if bc > ca && bc > ab {
        println!("{}", ca * ab / 2);
    }
    if ca > ab && ca > bc {
        println!("{}", ab * bc / 2);
    }
}