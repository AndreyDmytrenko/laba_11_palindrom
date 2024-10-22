fn is_palindrome(n: u32) -> bool {
    let original = n.to_string();
    let reversed: String = original.chars().rev().collect();
    original == reversed
}

fn main() {
    let data = [
        (123, false),
        (121, true),
        (1221, true),
    ];

    data
        .iter()
        .for_each(|(n, exp)| {
            assert_eq!(is_palindrome(*n), *exp);
            println!("{} -> Очікувано: {}, Отримано: {}", n, exp, is_palindrome(*n));
        });
}
