#[derive(Debug)]
struct Highlight<'document>(&'document str);
// struct Highlight( str);
fn erase(_: String) {

}


fn main() {
    let text = String::from("The quick brown fox jumps over the lazy dog.");

    let fox = Highlight(&text[4..19]);
    let dog = Highlight(&text[35..43]);

    erase(text);
    println!("{:?}", fox);
    println!("{:?}", dog);

   /* let magic1 = String::from("abracadabra!");
    let result;
    {
        let magic2 = String::from("shazam!");
        result = longest_word(&magic1, &magic2);
    }

    /*let magic1 = String::from("abracadabra!");
    let magic2 = String::from("shazam!");*/

    let result = longest_word(&magic1, &magic2);

    println!("The longest magic word is {}", result);*/
    /* let mut value = "hello".to_string();
     let ref1 = & value;
     let ref2 = &mut value;

    println!("{} {}", ref1,  ref2);*/

    /*  let mut greeting = String::from("hello");
    change(&mut greeting);
    println!("{}", greeting);*/

    /*let greeting = String::from("hello");
    let greeting_reference=&greeting;
    println!("{}", greeting);

    let s = String::from("Hello rust");

    process(&s);
    process(&s);*/
}

fn longest_word<'a>(x: &'a String, y: &'a String) -> &'a String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn change(message: &mut String) {
    message.push_str("!");
}

fn process(s: &String) {
    println!("from process {}!", s);
}
