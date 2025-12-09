use enum_as_str_derive::AsStr;

#[derive(AsStr)]
enum Color {
    Red,
    Green,
    Blue,
}

fn main() {
    let c = Color::Red;
    println!("{}", c.as_str());
}
