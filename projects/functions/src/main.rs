fn main() {
    println!("Hello, world!");

    let my_argument: i32 = 5;
    my_second_function(my_argument);

    let result = multiply(4, 5);
    println!("The multiply() function returned {result}");

    let mod2 = modulus_2(13);
    println!("mod2 = {mod2}");
}

fn my_second_function(x: i32) {
    println!("Hello, function number {x}!");
}

fn multiply(x: i32, y: i32) -> i32 {
    let result: i32 = x * y;
    println!("{x} * {y} = {result}");
    result
}

fn modulus_2(x: i64) -> i64 {
    x % 2
}