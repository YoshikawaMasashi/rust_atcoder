fn main() {
    let s = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim_right().to_owned()
    };

    let (t, x) = {
        let mut ws = s.split_whitespace();
        let t: i32 = ws.next().unwrap().parse().unwrap();
        let x: i32 = ws.next().unwrap().parse().unwrap();
        (t, x)
    };

    let t_f: f64 = t as f64;
    let x_f: f64 = x as f64;


    println!("{}", t_f / x_f);
}