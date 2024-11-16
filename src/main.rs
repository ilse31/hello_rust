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
}
