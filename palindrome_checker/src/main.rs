use std::io::{stdin};
fn main() {
    println!("Enter the string to check: ");
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

    //chars functions returns an array of only characters in the string
    // trim func removes the /r /n in the string
    let mut str = input.trim().clone().chars();
    println!("String now: {:?}",str);

    //string can only be sliced when converted into an array of bytes containing asci values of chars
    let s = input.as_bytes();
    println!("String now: {:?}",s);
    ///str is a char struct we cannot access the length of that
    /// therefore we create a veector of chars
    let char_vec: Vec<char> = str.clone().collect();;
    let n= char_vec.len();
    let mut count = 0;
    for i in 0..n / 2 {
        if s[i] != s[n - i - 1] {
            println!("{} != {}",s[i],s[n-i-1]);//traversing bytes
            count += 1;  
        };
        println!("{:?} is  a palindrome to {:?}",str.nth(i),str.nth(n-i-2));//ttraveresing chars
    }
    println!("total matched chars: {}",count);
}
