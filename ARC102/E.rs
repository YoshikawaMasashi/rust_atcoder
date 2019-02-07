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

struct CombinationGenerator {
    combination_memo : std::collections::HashMap<(i64, i64), i64>
}

impl CombinationGenerator {
    pub fn new() -> CombinationGenerator {
        let mut ret :CombinationGenerator = CombinationGenerator{combination_memo: std::collections::HashMap::new()};
        ret
    }

    pub fn combination(&self, n: i64, r: i64) -> i64 {
        if self.combination_memo.contains_key(&(n, r)) {
            *self.combination_memo.get(&(n, r)).unwrap()
        } else if n == r {
            1
        } else {
            let mut ret: i64 = self.combination(n, r + 1);
            ret *= (n - (n - r - 1));
            ret /= (n - r - 1 + 1);
            ret
        }
    }

    pub fn combination_with_repitition(&self, n: i64, r: i64) -> i64 {
        self.combination(n + r - 1, r)
    }
}

struct KeisuGenerator {
    _keisu_memo : std::collections::HashMap<i64, std::vec::Vec<i64>>
}

impl KeisuGenerator {
    pub fn new() -> KeisuGenerator {
        let mut ret :KeisuGenerator = KeisuGenerator{_keisu_memo: std::collections::HashMap::new()};
        ret
    }

    pub fn _keisu(&self, n: i64) -> std::vec::Vec<i64> {
        if self._keisu_memo.contains_key(&n) {
            (*self._keisu_memo.get(&n).unwrap()).to_vec()
        } else {
            let mae_keisu :std::vec::Vec<i64>;
            if n > 0 {
                mae_keisu = self._keisu(n - 1);
            } else {
                let mut mae_keisu_ = std::vec::Vec::new();
                mae_keisu_.push(1);
                mae_keisu = mae_keisu_;
            }

            let mut ret :std::vec::Vec<i64> = std::vec::Vec::new();
            for i in 0..(n+1) {
                if i == 0 || i == n {
                    ret.push(1);
                } else {
                    ret.push(mae_keisu[i as usize - 1] + mae_keisu[i as usize]);
                }
            }
            ret
        }
    }

    pub fn keisu(&self, n: i64) -> std::vec::Vec<i64> {
        let mut ret = self._keisu(n);
        for i in 0..ret.len() {
            if i % 2 == 1 {
                ret[i] = - ret[i];
            }
        }
        ret
    }
}

fn main() {
    let kn = read_ints();
    let k = kn[0];
    let n = kn[1];

    let combination_generator = CombinationGenerator::new();
    let keisu_generator = KeisuGenerator::new();

    for i in 2..(2*k+1) {
        let mut minus = 0;
        let mut minus_max = n / 2 + 1;
        let mut ret = 0;
        for j in 1..(i/2 + 1) {
            if 1 <= j && j <= k && 1 <= (i - j) && (i - j) <= k {
                minus += 1;
            }
        }
        let mut minus_keisu = keisu_generator.keisu(minus);
        for j in 0..(std::cmp::min(minus_keisu.len(), minus_max as usize)) {
            ret += minus_keisu[j] * combination_generator.combination_with_repitition(k, n - 2 * j as i64);
        }
        println!("{}", ret % 998244353);
    }

}