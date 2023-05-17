#![allow(non_snake_case)]

///
/// How to define a custom "iterator adapter".
///
/// Step 1: Define a struct for the custom adapter.
/// Step 2: Implement `Iterator` for the custom adapter.
/// Step 3: Define a new extension trait with the new operator to be
///         added, as a sub-trait of `Iterator`.
/// Step 4: Implement the adapter trait for all types that implement `Iterator`.
///         ("Blanket implementation" for all iterators.)
///
/// Extension traits are a programming pattern that makes it possible
/// to add methods to an existing type outside of the crate defining
/// that type.

mod iterator_adapter_Map {
    // Step 1: Define a struct for the custom adapter.

    // fn map<B, F>(self, f: F) -> Map<Self, F>
    // where
    //     Self: Sized,
    //     F: FnMut(Self::Item) -> B,
    // {
    //     Map::new(self, f)
    // }

    struct Map<I, F> {
        orig: I,
        f: F,
    }

    // Step 2: Implement `Iterator` for the custom adapter.

    impl<I, F, R> Iterator for Map<I, F>
    where
        I: Iterator,
        F: FnMut(I::Item) -> R,
    {
        type Item = R;

        fn next(&mut self) -> Option<Self::Item> {
            match self.orig.next() {
                Some(v) => Some((self.f)(v)),
                None => None,
            }
        }
    }

    // Step 3: Define a new extension trait with the new operator to be
    //         added, as a sub-trait of `Iterator`.
    trait MapExt: Iterator {
        fn fmap<F, R>(self, f: F) -> Map<Self, F>
        where
            F: FnMut(Self::Item) -> R,
            Self: Sized,
        {
            Map { orig: self, f }
        }
    }

    // Step 4: Implement the trait for all types that implement `Iterator`.
    impl<I: Iterator> MapExt for I {}

    /*
     * TODO
     */

    #[test]
    fn test() {
        let vs = vec![1, 2, 3, 4, 5];

        let result: Vec<_> = vs.into_iter().fmap(|x| x * 2).collect();

        assert_eq!(result, [2, 4, 6, 8, 10]);
    }
}

mod iterator_adapter_Unique {
    use std::collections::HashSet;
    use std::hash::Hash;

    // Step 1: Define a struct for the custom adapter.
    struct Unique<I>
    where
        I: Iterator,
        I::Item: Hash + Eq + Clone,
    {
        orig: I,
        seen: HashSet<I::Item>,
    }

    // Step 2: Implement `Iterator` for the custom adapter.

    impl<I> Iterator for Unique<I>
    where
        I: Iterator,
        I::Item: Hash + Eq + Clone,
    {
        type Item = I::Item;

        fn next(&mut self) -> Option<Self::Item> {
            loop {
                if let Some(v) = self.orig.next() {
                    if !self.seen.contains(&v) {
                        self.seen.insert(v.clone());
                        return Some(v);
                    }
                } else {
                    return None;
                }
            }
        }
    }

    // Step 3: Define a new extension trait with the new operator to be
    //         added, as a sub-trait of Iterator.

    trait UniqueExt: Iterator {
        fn unique(self) -> Unique<Self>
        where
            Self: Sized,
            Self::Item: Hash + Eq + Clone,
        {
            Unique {
                orig: self,
                seen: HashSet::new(),
            }
        }
    }

    // Step 4: Implement the trait for all types that implement `Iterator`.
    impl<I: Iterator> UniqueExt for I {}

    // #[cfg(feature = "skip")]
    #[test]
    fn test() {
        let vs = vec!["a", "b", "a", "cc", "cc", "d"];

        let result: Vec<_> = vs.into_iter().unique().collect();

        assert_eq!(result, ["a", "b", "cc", "d"]);
    }
}

// #[cfg(feature = "skip")]
mod iterator_adapter_Flatten {

    struct Flatten<I>
    where
        I: Iterator,
    {
        orig: I,
    }

    // Step 2: Implement Iterator for the custom adapter.

    /*
     * TODO
     */

    // Step 3: Define a new extension trait with the new operator to be
    //         added, as a sub-trait of Iterator.

    /*
     * TODO
     */

    // Step 4: Implement the trait for all types that implement Iterator.

    /*
     * TODO
     */

    #[test]
    fn test() {
        let vs = vec![vec![1, 2], vec![3, 4]];

        let result: Vec<_> = vs.into_iter().flatten().collect();

        assert_eq!(result, [1, 2, 3, 4]);
    }
}

/// The `FromIterator` trait allows for a collection to be built from an iterator.
/// By implementing `FromIterator` for a type, you define how it will be created
/// from an iterator. This is common for types which describe a collection of some kind.
mod from_iterator_in_std {
    pub trait FromIterator<A>: Sized {
        // Required method
        fn from_iter<I>(iter: I) -> Self
        where
            I: IntoIterator<Item = A>;
    }
}

#[test]
fn from_iter_demo() {
    // A sample collection, that's just a wrapper over `Vec<T>`
    #[derive(Debug)]
    struct MyCollection(Vec<i32>);

    // Let's give it some methods so we can create one and add things to it.
    impl MyCollection {
        fn new() -> MyCollection {
            MyCollection(Vec::new())
        }

        fn add(&mut self, elem: i32) {
            self.0.push(elem);
        }
    }

    // and we'll implement FromIterator
    impl FromIterator<i32> for MyCollection {
        fn from_iter<I: IntoIterator<Item = i32>>(iter: I) -> Self {
            let mut my_collection = MyCollection::new();

            for i in iter {
                my_collection.add(i);
            }
            my_collection
        }
    }

    // Now we can make a new iterator...
    let iter = 0..5;

    // ... and make a MyCollection out of it
    let collection = MyCollection::from_iter(iter);
    assert_eq!(collection.0, vec![0, 1, 2, 3, 4]);

    // collect works too!

    let iter = 0..5;
    let c = iter.collect::<MyCollection>();

    assert_eq!(c.0, vec![0, 1, 2, 3, 4]);
}

#[test]
fn from_iter_exercise() {
    //  [1, --]--->[2, --]---> [Nil]

    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    impl FromIterator<i32> for List {
        fn from_iter<T: IntoIterator<Item = i32>>(iter: T) -> Self {
            iter.into_iter()
                .fold(List::Nil, |acc, n| List::Cons(n, Box::new(acc)))
        }
    }
    // Nil 
    // Cons(2, Nil)
    // Cons(4, Cons(2, Nil))
    // Cons(6,  Cons(4, Cons(2, Nil)))

    /*
     * TODO: Implement `FromIterator` for `List`.
     */
    let src = vec![1, 2, 3];

    // Cannot directly collect to array.
    let list: [i32; 3] = src
        .iter()
        .map(|item: &i32| item * 2)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    // println!("list = {:?}", list);

    // Having implemented `FromIterator`, we can collect into a `List`.
    let list: List = src.iter().map(|item: &i32| item * 2).rev().collect();
    println!("list = {:?}", list);
}
