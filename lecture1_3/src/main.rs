 use std::io; // input output library
use rand::Rng; //generating random number
use  std::cmp::Ordering;
use colored::*;
pub struct Guess {
    value : i32,
}
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value should be b/w 1 & 100 ,got {}.",value);
        }
        Guess {value}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    // A should_panic test would pass even if the test panics for a different reason from the one we were expecting. To make should_panic tests more precise, we can add an optional expected parameter to the should_panic attribute.
    //#[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100(){
        Guess::new(200);
    }
}
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
