fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest_string(string1.as_str(), string2);
    // 最長の文字列は、{}です
    println!("The longest string is {}", result);
}

fn longest_string<'a>(s1: &'a str, s2: &'a str) -> &str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}