use std::io::{stdin};
fn main() {
    println!("Enter the number: ");
    let mut input = String::new();
    stdin().read_line(& mut input).expect("Invalid input");
    let mut num :i32 = input.trim().parse().expect("Internal error");
    let mut ans =1;
    while num>0 {
        ans=ans*num;
        num-=1;
    }
    println!("{:?}",ans);
}
