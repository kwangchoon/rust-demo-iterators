/// There are two common uses of iterators.
/// - "Adapters" create a new iterator based on a current iterator.
/// - "Consumers" consume an iterator to produce another value.

/**
 * Iterator Adapters are methods that produce a new iterator.
 * - map, flat_map, filter, filter_map, zip, chain, take, skip,...
 */
mod adapters {

    #[test]
    fn mapping() {
        let src = vec![1, 2, 3];

        let mut dest = Vec::new();
        for item in &src {
            // The essence of `map` is that for each item in your source list,
            // you're writing something into your destination list.
            dest.push(format!("{} doubled is {}", item, item * 2));
        }
        println!("dest          = {:?}", dest);

        // The same format expression appears here, but it's in a call to `map`.
        // The `collect` at the end takes the iterator and collects it into a `Vec`.
        let dest_with_map: Vec<String> = src
            .iter()
            .map(|item: &i32| format!("{} doubled is {}", item, item * 2))
            .collect();

        println!("dest_with_map = {:?}", dest_with_map);
    }

    // #[cfg(feature = "skip")]
    #[test]
    fn mapping_exercise_make_this_compile() {
        let player_scores = [("Jack", 20), ("Jane", 23), ("Jill", 18), ("John", 19)];

        let players = player_scores
            .into_iter()
            .map(|(player, _score)| player)
            .collect::<Vec<_>>();

        assert_eq!(players, ["Jack", "Jane", "Jill", "John"]);
    }

    #[test]
    fn iter_mut_and_mapping() {
        let mut teams = [
            [("Jack", 20), ("Jane", 23), ("Jill", 18), ("John", 19)],
            [("Bill", 17), ("Brenda", 16), ("Brad", 18), ("Barbara", 17)],
        ];

        let teams_in_score_order = teams
            .iter_mut()
            .map(|team| {
                team.sort_by(|&a, &b| a.1.cmp(&b.1).reverse());
                team
            })
            .collect::<Vec<_>>();

        println!("Teams: {:?}", teams_in_score_order);
    }

    #[test]
    fn reversing() {
        let vs = vec![
            String::from("red"),
            String::from("green"),
            String::from("blue"),
        ];

        // The `map` iterator implements `DoubleEndedIterator`, meaning that you
        // can also `map` backwards.
        let rs: Vec<String> = vs.iter().map(|x| x.to_ascii_uppercase()).rev().collect();
        println!("vs = {vs:?}, rs = {rs:?}");
    }

    #[test]
    fn map_with_mutable_effects() {
        let mut c = 0;

        ['a', 'b', 'c']
            .into_iter()
            .map(|letter: char| {
                c += 1;
                (letter, c)
            })
            .for_each(|pair| println!("{pair:?}"));
    }

    #[test]
    fn flatmapping() {
        let src = vec![1, 2, 3, 4];

        let mut dest = Vec::new();
        // In the imperative form, `flat_map` translates to a nested for loop.
        for i in src.iter() {
            for j in 0..*i {
                dest.push(j);
            }
        }
        println!("dest              = {:?}", dest);

        let dest_with_flatmap: Vec<i32> = src.iter().flat_map(|i: &i32| 0..*i).collect();
        println!("dest_with_flatmap = {:?}", dest_with_flatmap);
    }

    #[test]
    fn map_andthen_flatten_is_the_same_as_flat_map() {
        let vs = vec![1, 2, 3, 4];
        let vs = vec![1, 2, 3, 4];

        let mapped_and_flattened: Vec<_> = vs.iter().map(|i| 0..*i).flatten().collect();
        println!("after map and flatten: {mapped_and_flattened:?}");

        let flat_mapped: Vec<i32> = [1, 2, 3, 4].iter().flat_map(|i| (0..*i)).collect();
        println!("    after flat_mapped: {flat_mapped:?}");
    }

    #[test]
    fn enumerating() {
        let src = vec![
            String::from("red"),
            String::from("green"),
            String::from("blue"),
        ];

        let mut dest: Vec<String> = Vec::new();
        for i in 0..src.len() {
            // The imperative form forces you to index into your vector. It's
            // simple, but I've found this to be surprisingly error prone,
            // leading to index out of bounds errors or logic errors.
            dest.push(format!("src[{}] = {}", i, src[i]));
        }
        println!("dest                = {:?}", dest);

        // Using enumerate you get the same effect, but you don't run the risk
        // of using the wrong variable as your index, or accidentally going
        // outside the bounds of your vector. Also, this works with data
        // structures that you can't index into, like linked lists.
        let dest_with_enumerate: Vec<String> = src
            .iter()
            .enumerate()
            .map(|(i, item): (usize, &String)| format!("src[{}] = {}", i, item))
            .collect();

        println!("dest_with_enumerate = {:?}", dest_with_enumerate);
    }

    #[test]
    fn filtering() {
        let src = Vec::from_iter(1..=10);

        let mut dest: Vec<i32> = Vec::new();
        for item in src.iter() {
            // The essence of filter is that you have an if statement, and you
            // write to your destination inside the filter.
            if item % 2 == 0 {
                dest.push(*item);
            }
        }
        println!("dest             = {:?}", dest);

        let dest_with_filter: Vec<i32> =
            src.iter().filter(|&item| item % 2 == 0).copied().collect();

        println!("dest_with_filter = {:?}", dest_with_filter);
    }

    #[test]
    fn filter_and_map() {
        let src = vec!["one", "1", "3", "not a number"];

        let mut dest: Vec<i32> = Vec::new();
        for item in src.iter() {
            // This is a lot like a regular `map` followed by a `filter`.
            if let Ok(parsed) = item.parse() {
                dest.push(parsed);
            }
        }
        println!("dest                 = {:?}", dest);

        // `filter_map` yields onfly the values for which the supplied closure returns `Some(value)`.
        let dest_with_filter_map: Vec<i32> =
            src.iter().filter_map(|&item| item.parse().ok()).collect(); // Ok, Err => Some(v), None

        println!("dest_with_ilter_map = {:?}", dest_with_filter_map);
    }

    #[test]
    fn take() {
        let v = (0..).take(5);
        println!("v = {:?}", v.collect::<Vec<_>>());

        let v = std::iter::repeat(7).take(5);
        println!("v = {:?}", v.collect::<Vec<_>>());
    }

    #[test]
    fn zipping() {
        let src_numbers = vec![1, 2, 3];
        let src_words = vec!["one", "two", "three", "four"];

        let mut dest = Vec::new();
        for i in 0..src_numbers.len().min(src_words.len()) {
            // Indexing into two lists can be particularly error prone. What
            // if one happens to be longer than the other, for example?
            let num_item: i32 = src_numbers[i];
            let word_item: &str = src_words[i];
            dest.push(format!("{}: {}", num_item, word_item));
        }
        println!("dest          = {:?}", dest);

        // Notice how we can chain iterators, using `zip` to combine the two but
        // after that it's just a normal `map` and `collect`.
        let dest_with_zip: Vec<String> = src_numbers
            .iter()
            .zip(src_words.iter())
            .map(|(num_item, word_item): (&i32, &&str)| format!("{}: {}", num_item, word_item))
            .collect();

        println!("dest_with_zip = {:?}", dest_with_zip);
    }

    #[test]
    fn chaining() {
        let src1 = vec![1, 2, 3];
        let src2 = vec![4, 5, 6];

        // let mut dest = Vec::new();
        // // Without chain, you need this function body listed out twice.
        // for item in src1.iter() {
        //     dest.push(item * 2);
        // }
        // for item in src2.iter() {
        //     dest.push(item * 2);
        // }
        // println!("dest            = {:?}", dest);

        // This version separates the composing of the iterators from the map
        // that you wanted to do to them.
        let dest_with_chain: Vec<i32> = src1
            .iter()
            .chain(src2.iter())
            .map(|item| item * 2)
            .collect();

        println!("dest_with_chain = {:?}", dest_with_chain);
    }

    // Unfold ...
    #[test]
    fn fibonacci_numbers() {
        let fibo = std::iter::successors(Some((0, 1)), |&(a, b)| Some((b, a + b)))
            .map(|(v, _)| v)
            .take(20);

        println!("fibo = {:?}", fibo.collect::<Vec<_>>());
    }
}

/**
 * Iterator Consumers
 * - collect, fold, sum, max, min, all, any, find, position, nth, last, count, for_each, etc
 */

mod consumers {

    #[test]
    fn folding() {
        let src = vec![1, 2, 3, 4, 5, 6];

        // When you think "fold", think "accumulator variable"
        let mut sum: i32 = 0;
        for item in &src {
            sum += item
        }

        // This is functionally equivalent to
        // 0 + 1 + 2 + 3 + 4 + 5 + 6
        // (the 0 is important in case the list is empty)
        // It's a common convention to call the accumulator `acc`
        let sum_with_fold: i32 = src.iter().fold(0, |acc: i32, item: &i32| acc + item);

        // Summing is a common use case, so there's a fold shorthand just for
        // that in the standard library.
        let sum_with_sum: i32 = src.iter().sum();

        // Notice that we don't have a `collect` on these, because `fold` is
        // already consuming the iterator into the type that we want to end up
        // with.

        println!("sum           = {:?}", sum);
        println!("sum_with_fold = {:?}", sum_with_fold);
        println!("sum_with_sum  = {:?}", sum_with_sum);
    }

    #[test]
    fn any_and_all() {
        let src = vec![1, 2, 3, 4, 5, 6];

        let with_any: bool = src.iter().any(|item: &i32| *item > 3);

        println!("any = {}", with_any);

        let with_all: bool = src.iter().all(|item: &i32| *item > 3);

        println!("all = {}", with_all);
    }

    #[test]
    fn finding() {
        let src = vec![1, 2, 3, 4, 5, 6];

        let mut found: Option<&i32> = None;
        for item in &src {
            // This is a lot like a filter that stops after the first result.
            if *item > 3 {
                found = Some(item);
                break;
            }
        }
        println!("found           = {:?}", found);

        // single-liner
        let found_with_find: Option<&i32> = src.iter().find(|&item| *item > 3);

        println!("found_with_find = {:?}", found_with_find);
    }
}
