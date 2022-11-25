fn main () {
    println!("  -1 as a u8 is : {}", (-1i8) as u8);

    // Size of a variables in bytes
    let x = 1u32;
    println!("size of x in bytes: {}", std::mem::size_of_val(&x));

    // Inference
    let elem = 5u8;
    let mut vec = Vec::new();
    vec.push(elem);
    println!("{:?}", vec);

    // Aliasing
    type Inch = u64;
    let inches: Inch = 2 as u64;

    println!("Inches {}", inches);
}