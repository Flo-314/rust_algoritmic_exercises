fn main() {
    let reversed_string: String = reverse("cool");
    println!("{}", reversed_string);
}
pub fn reverse(input: &str) -> String {
    let mut reversed_string: String = String::new();

    for letter in input.chars().rev() {
        reversed_string.push(letter);
    }
    reversed_string
}
