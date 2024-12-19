use func::{func, func_once};

struct Test;

fn main() {
    let d = 32;
    let c = func!([d, c: d] |a: f32, b: i32| {
        println!("test {}", a);
        println!("test {}", d + c + b);
        0.0
    });

    let func_once = func_once!([d, _c: Test] |a, b: i32| {
        println!("test {}", a);
        println!("test {}", d + b);
        d
    });

    func_once.call(10.0, 11);
    // func_once.call(10.0, 11); // This won't compile

    println!("{}", c.call(10.0, 11));
}
