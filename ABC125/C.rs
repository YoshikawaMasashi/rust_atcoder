// まだできていない

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

fn all_prime_number() -> std::vec::Vec<i64> {
    let mut prime_number_vec_zero_padding = std::vec::Vec::new();
    prime_number_vec_zero_padding.push(0);
    prime_number_vec_zero_padding.push(0);
    for number in 2..100000 {
        prime_number_vec_zero_padding.push(number);
    }
    let mut number = 2;
    while(number * number < 100000) {
        if (prime_number_vec_zero_padding[number] > 0) {
            let mut not_prime_number = number*number;
            while(not_prime_number < 100000) {
                prime_number_vec_zero_padding[not_prime_number] = 0;
                not_prime_number += number;
            }
        }
        number += 1;
    }
    
    let mut prime_number_vec = std::vec::Vec::new();
    for number in 2..100000 {
        if prime_number_vec_zero_padding[number] > 0 {
            prime_number_vec.push(number as i64);
        }
    }
    return prime_number_vec;
}

fn main() {
    let n: i64 = read_ints()[0];
    let a: std::vec::Vec<i64> = read_ints();

    let prime_number_vec = all_prime_number();

    let mut prime_number_num: std::vec::Vec<std::vec::Vec<i64>> = std::vec::Vec::new();
    for row in 0..n {
        let mut prime_number_num_row: std::vec::Vec<i64> = std::vec::Vec::new();
        for col in 0..prime_number_vec.len() {
            prime_number_num_row.push(0);
        }
        prime_number_num.push(prime_number_num_row);
    }

    for row in 0..n {
        let mut a_i = a[row];
        for prime_number_idx in 0..prime_number_vec.len() {
            let prime_number = prime_number_vec[prime_number_idx]
            while(a_i % prime_number == 0) {
                prime_number_num[row][prime_number_idx] += 1;
                a_i = a_i / prime_number;
            }
            if a_i == 1 {
                break;
            }
        }
    }

    
}