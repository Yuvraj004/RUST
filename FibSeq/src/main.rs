use std::io::{stdin};// Bring both io module and stdin function into scope
// certain items like stdin are part of a module (std::io) and not directly a method that can be imported individually.
fn main() {
    println!("Enter the number of sequence: ");
    let mut input = String::new();
    stdin().read_line(& mut input).expect("Invalid input");
    let mut num :i32 = input.trim().parse().expect("Internal error");
    let mut prev =0;let mut next=1;let mut temp = next;
    while num >0 {
        print!(" {:?} ",prev);
        temp = next;
        next = prev +next;
        num=num-1;
        prev=temp;
        
    }

    // println!("{:?}",num);
}
