use func::func;

fn main() {
    let coeff = 3; // Captured variable
    let add_mul = func! { [coeff] | a, b | {
        let result = (a + b) * coeff;
        println!("Adding {} to {} and multiplying by {} = {}", a, b, coeff, result);
        result
    }};

    assert_eq!(add_mul.call(1, 2), 9);
}
