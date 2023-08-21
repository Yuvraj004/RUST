#[derive(PartialEq, Debug)]
struct Shoe{
    size :u32,
    style: String,
}
fn shoes_in_size(shoes: Vec<Shoe> , shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    //into_iter -> to create an interator that takes ownership of the vector
    //we call filter() to adapt that iterator into a new iterator that returns only the conditional elements.
}


#[cfg(test)]
mod tests {
    use super::*;
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();// v1_iter mutable becoz calling next changes internal state that the iterator uses to keep track of its own position in the sequence . OR next code consumes the iterator
        //no need to make _iter mutable with for loop becoz loop took ownership and made it mutable

        assert_eq!(v1_iter.next(), Some(&1));//calls to next are immutable references to the values of a vector
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
        // The Iterator trait only requires implementors to define one method: the next method, which returns one item of the iterator at a time wrapped in Some and, when iteration is over, returns None.
    }
    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();
        // We aren’t allowed to use v1_iter after the call to sum because sum takes ownership of the iterator we call it on, and iterates through  by calling next().
        assert_eq!(total, 6);
    }
}