fn main() {
    let apple = "hello".to_string();
    println!("{}",poop(apple));
}

fn poop(apple: String) -> String {
    return "hello".to_owned() + &apple;
}
