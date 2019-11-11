fn main() {
    (1..40)
        .map(|i| (i, fizz_buzz(i)))
        .for_each(|res| println!("{:?}", res));
}

// 3: fizz, 5: buzz, 15: fizzbuzz
fn fizz_buzz(n: usize) -> String {
    match (n % 3, n % 5) {
        (0, 0) => "FizzBuzz",
        (0, _) => "Fizz",
        (_, 0) => "Buzz",
        _ => "",
    }
    .to_string()
}
