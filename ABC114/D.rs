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

fn prime_numbers(n: i64) -> std::vec::Vec<i64> {
    let mut ret = std::vec::Vec::new();
    for i in 2..(n+1) {
        let mut is_prime = true;
        for j in 0..ret.len() {
            if i % ret[j] == 0 {
                is_prime = false;
            }
        }
        if is_prime {
            ret.push(i);
        }
    }
    return ret;
}

fn main() {
    let n: i64 = read_ints()[0];
    if n <= 9 {
        println!("0");
        return;
    }

    let primes = prime_numbers(n);

    let mut factor_table: std::vec::Vec<i64> = std::vec::Vec::new();
    for i in 0..primes.len() {
        factor_table.push(0);
    }

    for i in 2..(n+1) {
        let mut tmp: i64 = i;
        for j in 0..primes.len() {
            while tmp % primes[j] == 0 {
                factor_table[j] += 1;
                tmp /= primes[j];
            }
        }
    }

    let mut ret: i64 = 0;

    // 2 * 4 * 4
    for i in 0..(factor_table.len()-2) {
        for j in (i+1)..(factor_table.len()-1) {
            for k in (j+1)..factor_table.len() {
                if factor_table[i] >= 2 && factor_table[j] >=4 && factor_table[k] >= 4 {
                    ret += 1;
                }
                if factor_table[i] >= 4 && factor_table[j] >=2 && factor_table[k] >= 4 {
                    ret += 1;
                }
                if factor_table[i] >= 4 && factor_table[j] >=4 && factor_table[k] >= 2 {
                    ret += 1;
                }
            }
        }
    }
    // 2 * 24
    // 4 * 14
    for i in 0..(factor_table.len()-1) {
        for j in (i+1)..(factor_table.len()) {
            if factor_table[i] >= 2 && factor_table[j] >= 24 {
                ret += 1;
            }
            if factor_table[i] >= 24 && factor_table[j] >= 2 {
                ret += 1;
            }
            if factor_table[i] >= 4 && factor_table[j] >= 14 {
                ret += 1;
            }
            if factor_table[i] >= 14 && factor_table[j] >= 4 {
                ret += 1;
            }
        }
    }

    // 74
    for i in 0..factor_table.len() {
        if factor_table[i] >= 74 {
            ret += 1;
        }
    }

    println!("{}", ret);
}