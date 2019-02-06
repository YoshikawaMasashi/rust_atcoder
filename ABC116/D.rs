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

    let mut t = std::vec::Vec::new();
    let mut d = std::vec::Vec::new();
    for i in 0..n {
        let mut td = read_ints();
        t.push(td[0]);
        d.push(td[1]);
    }

    let mut dts :std::vec::Vec<(i64, i64)> = std::vec::Vec::new();
    
    for i in 0..n {
        dts.push((d[i as usize], t[i as usize]));
    }
    dts.sort();

    let mut eated :std::collections::HashMap<i64, std::vec::Vec<i64>> = std::collections::HashMap::new();
    let mut s_x = 0;
    for dt in dts.iter().rev().take(k as usize) {
        if !eated.contains_key(&dt.1) {
            eated.insert(dt.1, std::vec::Vec::new());
        }
        eated.get_mut(&dt.1).unwrap().push(dt.0);
        s_x += dt.0;
    }

    let mut uneated :std::collections::HashMap<i64, std::vec::Vec<i64>> = std::collections::HashMap::new();
    for dt in dts.iter().take((n - k) as usize) {
        if eated.contains_key(&dt.1) {
            continue;
        }
        if !uneated.contains_key(&dt.1) {
            uneated.insert(dt.1, std::vec::Vec::new());
        }
        uneated.get_mut(&dt.1).unwrap().push(dt.0);
    }

    let mut nakuseru :std::vec::Vec<i64> = std::vec::Vec::new();
    for eated_ in eated.iter_mut() {
        eated_.1.sort();
        for i in 0..eated_.1.len()-1 {
            nakuseru.push(eated_.1[i]);
        }
    }
    nakuseru.sort();

    let mut tsuketaseru :std::vec::Vec<i64> = std::vec::Vec::new();
    for uneated_ in uneated.iter_mut() {
        uneated_.1.sort();
        tsuketaseru.push(*uneated_.1.iter().rev().next().unwrap());
    }
    tsuketaseru.sort();

    let mut s_s :std::vec::Vec<i64> = std::vec::Vec::new();
    s_s.push(s_x);
    for i in 0..std::cmp::min(nakuseru.len(), tsuketaseru.len()) {
        let s = s_s[s_s.len() - 1] - nakuseru[i] + tsuketaseru[tsuketaseru.len() - i - 1];
        s_s.push(s);
    }

    let mut x: i64 = eated.len() as i64;
    let mut f_s_max = 0;
    for i in 0..s_s.len() {
        if f_s_max < (s_s[i] + (x + i as i64) * (x + i as i64)) {
            f_s_max = s_s[i] + (x + i as i64) * (x + i as i64);
        }
    }

    println!("{}", f_s_max);
}