#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn analyze_slice(slice: &[i32]) {
    println!("first element in this slice: {}", slice[0]);
    println!("the length of this slice: {}", slice.len());
}

fn main () {
    println!("1 - 2 = {}", 1i32 - 2);

    println!("One million is written as {}", 1_000_000u32);

    // Tuples elements
    let basic_tuple = (1u8, 1.3f32);
    println!("basic tuple first value: {}", basic_tuple.0);
    println!("basic tuple second value: {}", basic_tuple.1);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);

    // Array examples
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    println!("first element in this array: {}", xs[0]);
    println!("length of this array: {}", xs.len());

    println!("borrow a section of the array as a slice");
    analyze_slice(&xs);

    // Example of empty slice
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);

    // Iteration in array
    for i in 0..xs.len() + 1 {
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }
}