use std::cmp::PartialOrd;
#[derive(Debug)]
enum Result<T,E>{
    Ok(T),
    Err(E)
}
struct PointSingle<T>{
    x:T,y:T
}
//generic type in structs
struct Point<T,U>{
    x:T,
    y:U
}
//generic types in methods
impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
        x: self.x,
        y: other.y,
        }
    }
}

fn main() {
    let point1 = Point{ x:3.0,y:12};
    //using genric typing we can represent different data types within an struct
    let p1: Point<i32, f64> = Point { x: 5, y: 10.4 };
    let p2: Point<&str, char> = Point { x: "Hello", y: 'c' };
    let p3: Point<i32, char> = p1.mixup(p2);
    println! ("p3.x = {}, p3.y {}", p3.x, p3.y);

    let num_list = vec![12,34,54,56,1,2,3];
    let ans = get_largest(num_list);
    // println!("{}",ans);
    let char_list  = vec!['d','s','a','b','z'];
    // println!("{}",get_largest(char_list));
    // println!("point1 : {:?}", point1);
    // println!("Hello, world!");
}
//largest character and integer from their respective arrays using a single function
fn get_largest<T: Copy+PartialOrd> (num_list : Vec<T>) -> T {
    let mut largest = num_list[0];
    for element in num_list{
        if element > largest{
            largest = element;
        }
    }
    largest
} 
