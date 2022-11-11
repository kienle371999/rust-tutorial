fn main() {
    let visitor_list = ["Tom", "Jame", "Jago"];

    let name = "Jame";

    for visitor in &visitor_list {
        if visitor == &name {
            println!("Hello {}", visitor);
        }
        println!("Hi {}", visitor);
    }

    // println!("Hello, world 123!");
}
