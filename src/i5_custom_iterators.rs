#![allow(non_snake_case)]

mod Iterator_for_Counter {
    struct Counter {
        max: i32,
        // `count` tracks the state of this iterator.
        count: i32,
    }

    impl Counter {
        fn new(max: i32) -> Counter {
            Counter { count: -1, max }
        }
    }

    /*
     * Implement `Iterator` for `Counter`.
     */

    impl Iterator for Counter {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            self.count += 1;

            if self.count < self.max {
                Some(self.count)
            } else {
                None
            }
        }
    }

    // #[cfg(feature = "skip")]
    #[test]
    fn test1() {
        let counter = Counter::new(10);
        for i in counter {
            println!("{i}");
        }
    }
}

mod IntoIterator_for_Counter {
    struct Counter {
        max: i32,
        // No need to track the state, because this isn't an iterator.
    }

    impl Counter {
        fn new(max: i32) -> Counter {
            Counter { max }
        }
    }

    /*
     * Implement `IntoIterator` for `Counter`.
     */
    impl IntoIterator for Counter {
        type Item = i32;
        type IntoIter = IntoIterX;

        fn into_iter(self) -> Self::IntoIter {
            IntoIterX {
                max: self.max,
                count: 0,
            }
        }
    }

    struct IntoIterX {
        count: i32,
        max: i32,
    }

    impl Iterator for IntoIterX {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            self.count += 1;

            if self.count < self.max {
                Some(self.count)
            } else {
                None
            }
        }
    }

    // #[cfg(feature = "skip")]
    #[test]
    fn test() {
        let counter = Counter::new(10);
        for i in counter {
            println!("{i}");
        }
    }
}

/**
 * Generate passowords of length `length`.
 */

mod Iterator_for_PasswordGenerator {
    use rand::Rng;

    struct PasswordGenerator {
        length: usize,
    }

    impl PasswordGenerator {
        fn new(length: usize) -> Self {
            Self { length }
        }
    }

    /*
     * Implement Iterator for PasswordGenerator.
     */

    impl Iterator for PasswordGenerator {
        type Item = String;

        fn next(&mut self) -> Option<Self::Item> {
            let mut password = String::new();

            for _ in 0..self.length {
                password.push((b'a' + rand::thread_rng().gen_range(0..=b'z' - b'a')) as char);
            }
            Some(password)
        }
    }

    // #[cfg(feature = "skip")]
    #[test]
    fn test() {
        let gen = PasswordGenerator::new(10);

        // `IntoIterator` is automatically generated for Iterator via blanket impl.
        for password in gen.take(10) {
            println!("{password:?}");
        }
    }
}

mod IntoIterator_for_PasswordGenerator {
    use rand::Rng;

    struct PasswordGenerator {
        length: usize,
    }

    impl PasswordGenerator {
        fn new(length: usize) -> Self {
            Self { length }
        }
    }

    /*
     * Implement `IntoIterator` for `PasswordGenerator`.
     */

    impl IntoIterator for PasswordGenerator {
        type Item = String;
        type IntoIter = IntoIterX;

        fn into_iter(self) -> Self::IntoIter {
            IntoIterX {
                length: self.length,
            }
        }
    }

    struct IntoIterX {
        length: usize,
    }

    impl Iterator for IntoIterX {
        type Item = String;

        fn next(&mut self) -> Option<Self::Item> {
            let mut password = String::new();

            for _ in 0..self.length {
                password.push((b'a' + rand::thread_rng().gen_range(0..=b'z' - b'a')) as char);
            }
            Some(password)
        }
    }

    // #[cfg(feature = "skip")]
    #[test]
    fn test() {
        let gen = PasswordGenerator::new(10);

        for password in gen.into_iter().take(10) {
            println!("{password:?}");
        }
    }
}
