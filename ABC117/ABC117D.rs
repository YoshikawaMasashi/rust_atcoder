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
    let nk = read_ints();
    let n = nk[0];
    let k = nk[1];
    let a_s = read_ints();

    let mut bits_count = [0; 63];
    for a in a_s {
        for i in 0..63 {
            bits_count[i] += a >> i & 1;
        }
    }
    let mut k_bits = [0; 63];
    for i in 0..63 {
        k_bits[i] += k >> i & 1;
    }

    let mut ret: i64 = 0;
    let mut b_decided = false;

    for i in (0..63).rev() {
        if b_decided {
            if bits_count[i] >= n - bits_count[i] {
                ret += (1 << i) * (bits_count[i]);
            } else {
                ret += (1 << i) * (n - bits_count[i]);
            }
        } else {
            if k_bits[i] == 0 {
                ret += (1 << i) * (bits_count[i]);
            } else if bits_count[i] >= n - bits_count[i] {
                ret += (1 << i) * (bits_count[i]);
                b_decided = true;
            } else {
                ret += (1 << i) * (n - bits_count[i]);
            }
        }
    }

    println!("{}", ret);
}