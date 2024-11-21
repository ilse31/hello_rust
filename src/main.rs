fn main() {
    println!("Hello, world!");

    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
    let message_number = 1;
    let message = if message_number == 1 {
        "Hello"
    } else {
        "Goodbye"
    };

    println!("The message is: {}", message);
    let x = x + 1;
    println!("The value of x is: {}", x);

    let string = "test";
    let string = string.len();
    println!("The length of the string is: {}", string);

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {}", result);

    let min_type_i32 = i128::MIN;

    println!("The minimum value of i8 is: {}", min_type_i32);

    let pointer123 = &5;
    println!("The value of pointer123 is: {}", pointer123);

    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    if let 12 = 5 {
        println!("The value is 12");
    } else {
        println!("The value is not 12");
    }

    let arraydata = [1, 2, 3, 4, 5];
    for element in arraydata.iter() {
        println!("The value of element is: {}", element);
    }

    for number in (1..4).rev() {
        println!("The value of number is: {}", number);
    }
}
