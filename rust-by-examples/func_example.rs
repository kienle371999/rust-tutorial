fn main() {
    fn function(i: i32) -> i32 { i + 1 }
    println!("function: {}", function(14));

    fn is_odd(n: u32) -> bool {
        n % 2 == 1
    }

    let upper = 5;
    let sum_of_squared_odd_numbers: u32 =
        (0..).map(|n| n * n)                             // All natural numbers squared
            .take_while(|&n_squared| n_squared < upper) // Below upper limit
            .filter(|&n_squared| is_odd(n_squared))     // That are odd
            .sum();                                     // Sum them
    println!("functional style: {}", sum_of_squared_odd_numbers);
}