#[cfg(test)]
mod tests {
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
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
        // We aren’t allowed to use v1_iter after the call to sum because sum takes ownership of the iterator we call it on.
        assert_eq!(total, 6);
    }
}