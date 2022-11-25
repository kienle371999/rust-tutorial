// An attrubute to hide warnings for unused code.
#![allow(dead_code)]

// Struct types
#[derive(Debug)]
struct Person {
    name: String,
    age: u8
}

#[derive(Debug)]
struct Pair(i32, u32);

// Enum types
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

fn inspect (event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the `enum`.
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // Destructure `Click` into `x` and `y`.
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}

// Self implementation for enum
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

fn main() {
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };
    println!("{:?}", peter);

    let pair = Pair(-5, 6);
    println!("{:?}", pair);

    inspect(WebEvent::Click { x: 50, y : 80 });

    // const data 
    static LANGUAGE : &str = "Rust language";
    const THRESHOLD : i32 = 10;

    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
}