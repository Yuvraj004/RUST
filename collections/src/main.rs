#![ allow(unused)]
// #[derive(debug)]
fn vectors() {
    println!("Hello, world!");
    let mut v:Vec<i32> = Vec::new();
    //we have to specify thhe data type when creating vectors
    //dynamic size
    v.push(1);v.push(1);v.push(4);
    //to initialize with values
    {let _v2  = vec![1,2,3];}//no need to specify data type
    //v2 will be dropped outsidde the scope
    //accessing vaalues
    let _third = &v[2]; //immutable reference to vec
    // v.push(6);//mutable reference
    //when extending memoryy vector ismoved to new location due to which
    //the _third pointer's location is changed
    println!("element is {}",_third);

    //syntax so that no error is returned
    //using the get keyword
    match v.get(20) {
        Some (_third) => println!("The third element is {}", _third),
        None => println!("There is no third element."),
    }
}

use unicode_segmentation::UnicodeSegmentation;
fn  strings(){
    //utf-8 encoded bytes
    let s1: String = String:: from("Hello, ");
    let s2: String = String::from("world!");
    let s3: String = format! ("{}{}", s1, s2);//format does not take ownerrship of the string
    //indexing
    let hello: String = String :: from("नमस्ते");
    // Bytes
    for b in "नमस्ते".bytes() {println!("{}",b);}
    // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164
    // Scalar values
    for c in "नमस्ते".chars() {println!("{}",c);}
    // ['7', 'A', '7', 'Q', '7', '0']
    // Grapheme clusters
    //no default method -> import crate unicode-segmentation
    for g in "नमस्ते".graphemes(true){
        println!("{}",g);
    }

    // ["3", "7", "X", "à"]
    
}

use std::collections::HashMap;
fn hashmaps(){
    let blue: String = String :: from("Blue");
    let yellow: String = String::from("Yellow");
    let mut scores: HashMap<String, i32> = HashMap:: new();
    //inserting
    scores.insert( blue,  10);//ownership is transferred
    scores.insert( yellow,  50);
    let team_name: String = String :: from("Blue");
    let score: Option<&i32> = scores.get(&team_name);
    //gettting
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    let text = "hello world beautiful world";
    let mut map = HashMap::new();
// ["hello", "world", "wonderful, "world"]
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert( 0);
        *count += 1;
    }
    println!("{:?}", map);
}
fn main(){
    vectors();
    strings();
    hashmaps();
}