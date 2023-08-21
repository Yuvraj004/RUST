pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}

fn main() {
    println!("Hello, world!");
    let v1 = vec!{1,2,3};
    let v_iter = v1.iter(); //iterator to v1 vector
    for val in v_iter {
        // println!("Got: {}",val);
    }

    // map returns a new iterator that produces the modified items. The closure here creates a new iterator in which each item from the vector will be incremented by 1:
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    // collect method consumes the iterator and collects the resulting values into a collection data type.

    assert_eq!(v2,vec![2,3,4]);

}
