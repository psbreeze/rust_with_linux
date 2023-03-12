fn main() {
    let s1 = String::from("Hello");
    let s2 = String::from("world");

    let result = longest(&s1, &s2);
    println!("The longest string is '{}'", result);
}

// 런타임 시점에 판단하여 빌림을 반환하는 케이스
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
