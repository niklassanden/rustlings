// enums1.rs
// No hints this time! ;)

#[derive(Debug, PartialEq)]
enum Message {
    Quit,
    Echo,
    Move,
    ChangeColor(u32),
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor(5));
}
