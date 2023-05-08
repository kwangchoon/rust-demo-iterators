///
/// Iterators for for-loop:
///
/// If you wish to allow your type T to be usable in a for loop, you have two options:
/// 1. impl Iterator for T, or
/// 2. impl IntoIterator for T
///

mod iterators_for_arrays {
    /**
     * Arrays implement `IntoIterator` (implemented for [T; N], &[T; N] and &mut [T; N]).
     */
    #[rustfmt::skip]
    #[test]
    fn iteration_over_arrays() {
        let array = [ 
            String::from("red"),
            String::from("green"),
            String::from("blue"),
        ];

        // into_iter() is called
        for i in array { // Change to &, and &mut
            println!("i = {i}");
        }

        // println!("array = {array:?}");
    }

    #[test]
    fn iteration_over_arrays_using_iter() {
        let array = [
            String::from("red"),
            String::from("green"),
            String::from("blue"),
        ];

        for i in array.iter() {
            println!("i = {i}");
        }

        println!("array = {array:?}");
    }

    #[test]
    fn iteration_over_arrays_using_iter_mut() {
        let mut array = [1, 2, 3];

        for i in array.iter_mut() {
            *i += 100;
        }

        for i in array.iter() {
            println!("i = {i}");
        }
    }
}

mod iterators_for_vector {
    /**
     * Vectors implement `IntoIterator` (implemented for Vec<T>, &Vec<T> and &mut Vec<T>).
     */
    #[rustfmt::skip]
    #[test]
    fn iteration_over_vector() {
        let vs = vec![
            String::from("red"),
            String::from("green"),
            String::from("blue"),
        ];

        // into_iter() is called
        for v in vs { // Change to &, and &mut
            println!("v = {}", v.to_ascii_uppercase());
        }

        // println!("vs = {vs:?}");
    }

    #[test]
    fn iteration_over_vector_using_iter() {
        let vs = vec![
            String::from("red"),
            String::from("green"),
            String::from("blue"),
        ];

        for v in vs.iter() {
            println!("v = {}", v.to_ascii_uppercase());
        }

        println!("vs = {vs:?}");
    }

    #[test]
    fn iteration_over_vector_using_iter_mut() {
        let mut vs = vec![
            String::from("red"),
            String::from("green"),
            String::from("blue"),
        ];

        for v in vs.iter_mut() {
            v.make_ascii_uppercase();
            println!("v = {v}");
        }

        println!("vs = {vs:?}");
    }
}
