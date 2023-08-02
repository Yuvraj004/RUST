 use std::io; // input output library
use rand::Rng; //generating random number
use  std::cmp::Ordering;
use colored::*;
 fn main() {
    println!("Guess Number!");
    

    

    let secret_num = rand::thread_rng().gen_range(1, 101);


    println!("The secret number is : {}",secret_num);
    loop {
        println!("Please input your guess.");
        let mut guess : String = String::new(); //by default variables in rust are immutable therefore we use mut keyword

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    let guess:u32 = match guess.trim().parse(){
        Ok(num) =>num,
        Err(_) => continue,
    };
    println!("You guessed : {}",guess);

    match guess.cmp(&secret_num){
        Ordering::Equal => {println!("{}","You win!".green());break;},
        Ordering::Greater => println!("{}","Too big!".red()),
        Ordering::Less => println!("{}","Too small!".red()),
    };
    }
}
