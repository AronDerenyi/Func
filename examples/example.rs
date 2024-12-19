use func::func;

fn main() {
    let d = 32;
    let c = func!([d, c: d] |a: f32, b: i32| {
        println!("test {}", a);
        println!("test {}", d + c + b);
        0.0
    });

    println!("{}", c.call(10.0, 11));
}
