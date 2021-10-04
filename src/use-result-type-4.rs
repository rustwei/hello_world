struct Person {
    first: String,
    last: String,
    middle: Option<String>,
}
fn build_full_name(persion: &Person) -> String {
    let mut full_name = String::new();
    full_name.push_str(&persion.first);
    full_name.push_str(" ");

    match &persion.middle {
        None => (),
        Some(middle_name) => {
            full_name.push_str(middle_name);
            full_name.push_str(" ");
        }
    };

    full_name.push_str(&persion.last);
    full_name
}
fn main() {
    let john = Person {
        first: String::from("James"),
        middle: Some(String::from("Oliver")),
        last: String::from("Smith"),
    };
    assert_eq!(build_full_name(&john), "James Oliver Smith");

    let alice = Person {
        first: String::from("Alice"),
        middle: None,
        last: String::from("Stevens"),
    };
    assert_eq!(build_full_name(&alice), "Alice Stevens");

    let bob = Person {
        first: String::from("Robert"),
        middle: Some(String::from("Murdock")),
        last: String::from("Jones"),
    };
    assert_eq!(build_full_name(&bob), "Robert Murdock Jones");
}
