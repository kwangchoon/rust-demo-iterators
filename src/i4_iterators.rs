use std::ops::Range;

/**
 * Rust allows a program to iterate over any type T in a for loop,
 * as long as T implements the `Iterator` trait.
 */
mod std_iterator_traits {

    /**
     * `Iterator` defines the interface for an iterator. When a caller
     * invokes `next()`, the iterator returns `Some(_)` if it has more items,
     * or `None` if it has exhausted all its items.
     */
    pub trait Iterator {
        type Item;

        fn next(&mut self) -> Option<Self::Item>;
    }

    /**
     * `IntoIterator` defines the interface for creating an Iterator.
     */
    pub trait IntoIterator {
        type Item;
        type IntoIter: Iterator<Item = Self::Item>;

        fn into_iter(self) -> Self::IntoIter;
    }

    /**
     * Blanket implementation for all iterators.
     *
     * It instructs Rust to automatically implement `IntoIterator` for any type
     * implements `Iterator`.
     */
    impl<I: Iterator> IntoIterator for I {
        type Item = I::Item;
        type IntoIter = I;

        fn into_iter(self) -> I {
            self
        }
    }
}

#[rustfmt::skip]
#[test]
fn iterator_using_into_iter() {
    let ages = [27, 35, 40];

    // create an iterator
    let mut iterator = ages.into_iter(); // iterator is lazy
                                         // display the iterator
    println!("{iterator:?}");            // IntoIter([27, 35, 40])
                                         // display each element in array
    assert_eq!(iterator.next(), Some(27));   
    assert_eq!(iterator.next(), Some(35));  
    assert_eq!(iterator.next(), Some(40));  
    assert_eq!(iterator.next(), None);   // None
                                         // display the iterator
    println!("{iterator:?}");            // IntoIter([])
                                         // display the array
    println!("ages = {ages:?}");         // [27, 35, 40] Why not moved???
}

#[rustfmt::skip]
#[test]
fn iterator_using_iter() {
    let colors = ["red", "green", "blue"];
    // create an iterator
    let mut iterator = colors.iter();    // iterator is lazy
                                       // display the iterator
    println!("{iterator:?}");          // Iter(["red", "green", "blue"])
                                       // display each element in array
    assert_eq!(iterator.next(), Some(&"red"));   
    assert_eq!(iterator.next(), Some(&"green"));  
    assert_eq!(iterator.next(), Some(&"blue"));  
    assert_eq!(iterator.next(), None);  
                                       // display the iterator
    println!("{iterator:?}");          // Iter([])
                                       // display the array
    println!("colors = {colors:?}");   // ["red", "green", "blue"]
}

#[test]
fn range_is_an_iterator_by_itself() {
    // See what happens if you use (0..10).into_iter()
    for i in 0..10 {
        println!("i = {i}");
    }
    // is a syntaxctic sugar for:
    let mut range = 0..10;
    loop {
        match range.next() {
            Some(x) => {
                println!("{x}");
            }
            None => {
                break;
            }
        }
    }
}

/**
 * The expression immediately following `in` must implement either
 * the `IntoIterator` trait, or the `Iterator` trait.
 */
#[test]
fn iterator_after_in() {
    let vs = 0..=3; // move
    for v in vs {
        println!("{v}");
    }

    let mut vs = 0..=3; // move
    while let Some(v) = vs.next() {
        println!("{v}");
    }

    let vs = [1, 2, 3]; // move
    for v in vs {
        println!("{v}");
    }

    let mut vs = [1, 2, 3].into_iter();
    while let Some(v) = vs.next() {
        println!("{v}");
    }

    let vs = vec![1, 2, 3]; // move
    for v in vs {
        println!("{v}");
    }

    let mut vs = vec![1, 2, 3].into_iter();
    while let Some(v) = vs.next() {
        println!("{v}");
    }
}
