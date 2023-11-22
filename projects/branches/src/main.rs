fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let new_number: i32 = if 5 > 8 { 5 } else { 6 };
    println!("new_number = {new_number}");

    let newer_number: i32 =
    if number > 6 {
        33
    } else {
        let t3: i32 = 44;
        let t3 = t3 + 39;
        t3
    };
    println!("newer_number = {newer_number}");

    let number_as_string =
    if number > 6 {
        "33".to_owned()
    } else {
        let t3: i32 = 44;
        let t3 = t3 + 39;
        let t3: String = t3.to_string();
        t3
    };
    println!("number_as_string = {number_as_string}");

    // loops

    let mut i: u8 = 0;
    loop {
        println!("loop index: {i}!");
        i += 1;
        if i > 254 {
            break;
        }
    }

    // loop with returned value
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    // loop labels
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // while loop
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // for loop
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    // for loop with spread and reverse
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
