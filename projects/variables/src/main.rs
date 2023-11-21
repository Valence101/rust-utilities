fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "   ";
    println!("The value of spaces is: '{spaces}'");
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");

    let my_tuple: (i32, f64, u8) = (300, 8.8, 21);

    let three_hundred = my_tuple.0;

    println!("The value of three_hundred is: {three_hundred}");

    let my_array: [i32; 5] = [1, 3, 44, 34, 44];

    let third_element = my_array[2];
    println!("my_array element 3 is: {third_element}");
}
