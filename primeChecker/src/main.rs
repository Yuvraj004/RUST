use std ::io::{stdin};
fn main() {
    println!("Enter the no. to check for prime!");
    let mut input  = String::new();
    stdin().read_line(& mut input).expect("Invalid Input!");
    let n :i32 = input.trim().parse().expect("Internal Error!");
    //dividing by every number till half of it
    let mut is_prime = true;
    let mut i =2;
    while i<=n/2 {
        if n % i ==0 {
            is_prime =false;
            break;
        }
        i=i+1
    }
    if is_prime == false {println!("{:?} is not Prime",n);}
    else {println!("{:?} is Prime",n);}
    
}
