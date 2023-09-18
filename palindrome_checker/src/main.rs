use std::io::stdin;
fn main() {
    print!("Enter the string to check: ");
    let mut input = String::new();
    stdin().read_line(& mut input).expect("Invalid Input!");
    // let length= input.len();
    // let mid = (input.len())/2;
    // let i =0;
    // while i<mid {
    //     if input[i] != input[length - i-1] {
    //         println!("{:?} is not palindrome",input);
    //         break;
    //     }
    //     i=i+1;
    // }
    let str = input.clone();
    let s = input.as_bytes();
    let n= s.len();
    let mut count = 0;
    for i in 0..n / 2 {
                if s[i] != s[n - i - 1] {
                    count += 1;
                    println!("{:?} is not a palindrome",str);
                };
            }
    println!("total matched chars: {}",count);
}
