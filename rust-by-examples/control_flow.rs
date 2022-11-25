fn main () {
    let n = 5;

    let big_n =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");
            10 * n
        } else {
            println!(", and is a big number, halve the number");
            n / 2
        };

    println!("Big n {}", big_n);

    // While statement
    let mut n = 1;
    while n < 10 {
        println!("Buzz -- {}", n);
        n += 1;
    }

    // For statement
    for n in 1..5 {
        println!("Buzz in for statement -- {}", n);
    }

    let mut names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }
    println!("names: {:?}", names);

    // Match statement
    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
    };

    println!("Binary statement {}", binary);

    let optional: Option<i32> = None;

    match optional {
        Some(i) => {
            println!("This is a really long string and `{:?}`", i);
        },
        _ => {
            println!("This is null");
        },
    };

   let number = Some(512);
   if let Some(i) = number {
        println!("Matched {:?}!", i);
    }
}