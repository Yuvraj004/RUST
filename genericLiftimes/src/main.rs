//generic lifetimes to help the burrow checker deciding the lifetime of the prameters that are a input to a function or method

fn main() {
    let string1: String = String:: from("abcd");
    let string2: String String:: from("xyz");
    let result: &str = longest(x: string1.as_str(), y: string2.as_str());
    println!("The longest string is {}", result);
    main2();
}

    //&i32 // reference
    //&'a i32 //reference with an explicit lifetime
    //&'a mut i32 // mutable reference with an explicit lifetime
fn longest<'a>(x: &'a str, y: &'a str) -> &str {
    //  Rust can’t tell whether the reference being returned refers to x or y. Actually, we don’t know either, because the if block in the body of this function returns a reference to x and the else block returns a reference to y
    // The borrow checker can’t determine this either, because it doesn’t know how the lifetimes of x and y relate to the lifetime of the return value. 
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// 'a. In practice, it means that the lifetime of the reference returned by the longest function is the same as the smaller of the lifetimes of the values referred to by the function arguments.
//Note that the longest function doesn’t need to know exactly how long x and y will live, only that some scope can be substituted for 'a that will satisfy this signature.
//
//
fn main3() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);//ERROR becoz borrowed value does not live long enough
}
//
//correct implementation of above main3
//
fn main2() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}
//
//
//
//Lifetime Annotations in Struct Definitions
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
// This annotation means an instance of ImportantExcerpt can’t outlive the reference it holds in its part field.
//
//
//
// Lifetime Ellisions
//The patterns programmed into Rust’s analysis of references are called the lifetime elision rules. These aren’t rules for programmers to follow; they’re a set of particular cases that the compiler will consider, and if your code fits these cases, you don’t need to write the lifetimes explicitly.
//
//The first rule is that the compiler assigns a lifetime parameter to each parameter that’s a reference. In other words, a function with one parameter gets one lifetime parameter: fn foo<'a>(x: &'a i32); a function with two parameters gets two separate lifetime parameters: fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on.
//
//The second rule is that, if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: 
//fn foo<'a>(x: &'a i32) -> &'a i32.
//
//The third rule is that, if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters. This third rule makes methods much nicer to read and write because fewer symbols are necessary.
//
//Because the third rule really only applies in method signatures, we’ll look at lifetimes in that context next to see why the third rule means we don’t have to annotate lifetimes in method signatures very often.
//
//impl<'a> ImportantExcerpt<'a> {
//     fn announce_and_return_part(&self, announcement: &str) -> &str {
//         println!("Attention please: {}", announcement);
//         self.part
//     }
// }
// There are two input lifetimes, so Rust applies the first lifetime elision rule and gives both &self and announcement their own lifetimes. Then, because one of the parameters is &self, the return type gets the lifetime of &self, and all lifetimes have been accounted for.

//
//
//The Static Lifetime
// One special lifetime we need to discuss is 'static, which denotes that the affected reference can live for the entire duration of the program. All string literals have the 'static lifetime, which we can annotate as follows:

// let s: &'static str = "I have a static lifetime.";
// The text of this string is stored directly in the program’s binary, which is always available. Therefore, the lifetime of all string literals is 'static.
