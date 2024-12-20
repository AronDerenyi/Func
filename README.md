# Func!

This crate introduces the `Func` struct which tries to work the same way closures do,
but with the captured data and the function body separated. This is achieved by storing
the captured data with a function pointer. It can make closures more transparent and
addresses multiple inconveniences with the default closure behaviour.

```rust
let coeff = 3;
let add_mul = func! { [coeff] | a, b | {
    println!("Adding {} to {} and multiplying by {}", a, b, coeff);
    (a + b) * coeff
}};

assert_eq!(add_mul.call(1, 2), 9);
```

## Trait implementations
Even though `Copy` and `Clone` closures have been a part of rust since 2018 there
are many standard traits that are not implemented for them. Namely `PartialEq`,
`Eq`, and `Hash`.

These traits are implemented for `Func` as long as the captured data (and also the
function pointer) implements them. Although `PartialEq` and `Eq` is not a 100% reliable
for function pointers it's really useful for caching and optimization purposes.

### Example:
```rust
fn multiply_by(by: i32) -> Func<(i32,), (i32,), i32> {
    func!([by] | value | { value * by })
}

fn test() {
    let mul_by_2 = multiply_by(2);
    assert_eq!(mul_by_2.call(10), 20);

    assert_eq!(mul_by_2, multiply_by(2));
    assert_ne!(mul_by_2, multiply_by(5));
}
```

## Explicit captures
Since function pointers cannot capture their environment it's necessary to explicitly
state each value that's captured by the `Func`. This makes reading the code easier and
avoids accidental captures.

### Example of accidental self capture:
Let's say we want to create a `'static` closure that prints the value of a field of a struct.

This accidentally captures a reference to self which if not `'static` would cause a compilation error.
```rust
|| println!("{}", self.value);
```

This works but requires unnecessary boilerplate.
```rust
{
    let value = self.value;
    move || println!("{}", value);
}
```

This is the same functionality as above but with less boilerplate thanks to the `func!` macro.
```rust
func!([value: self.value] { println!("{}", value) });
```

### Example of capturing clone:
Let's say we want to capture a cloned value without moving or capturing a reference.

This accidentally captures a reference to the value itself.
```rust
|| println!("{}", value.clone());
```

This works but requires unnecessary boilerplate.
```rust
{
    let value = value.clone();
    move || println!("{}", value);
}
```

This is the same functionality as above but with less boilerplate thanks to the `func!` macro.
```rust
func!([value: value.clone()] { println!("{}", value) });
```

## Func, FuncMut, and FuncOnce
Just like `Fn`, `FnMut`, and `FnOnce`, `Func` has three different versions. `Func` passes
the captured values by reference to the function body making it possible to be called
multiple times. `FuncMut` passes the captured values by mutable reference to the function
body allowing the body to mutate the captured values. `FuncOnce` moves the captured values
into the function body making it impossible to call the function more than once. They also
have their own macros too for easier creation (`func!`, `func_mut!`, and `func_once!`).

## To fn
`Func`, `FuncMut`, and `FuncOnce` all have their own `to_fn`, `to_fn_mut`, and `to_fn_once`
functions that consumes the `Func` and returns an `Fn` trait. This is useful when you
want to pass a `Func` to an external crate that only accepts `Fn` traits.

# Disclaimer
This crate is still in an early stage of development and is not recommended for production use.
Any feedback or suggestion is appreciated.
