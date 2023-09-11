use std::io;
fn main() {
    println!("Hello, world!");
    println!("Enter a string: ");
    let mut user_input = String::new();
    let stdin = io::stdin(); //io::stdin() Constructs a new handle to the standard input of the current process.
    let _ = stdin.read_line(&mut user_input); //return a result therfore we use "let _" to just recieve the result

    println!("input = {}",user_input);

    println!("Enter a num: ");
    let mut num_string = String::new();
    io::stdin()
        .read_line(&mut num_string)
        .expect("failed  to read input");
    // let num :i32 = num_string.trim().parse().expect("invalid input");
    // let num = num_string.trim().parse::<i32>().expect("invalid input");
    //To check the data type of a variable just try to run a undefined method on that variable. Like this : num.what_is_this();
    if let Ok(num) = num_string.trim().parse::<i32>() {
        println!("{:?}", num);
    }
    // println!("{:?}",num);
}
