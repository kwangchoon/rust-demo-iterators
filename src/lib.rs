#![allow(unused)]

mod i1_arrays_and_vectors;
mod i2_std_iterators;
mod i3_mini_project;
mod i4_iterators;
mod i5_custom_iterators;
mod i6_iterator_adapters;

#[macro_export]
macro_rules! delim {
    () => {
        println!("{}", "-".repeat(50));
    };
    ($len:expr) => {
        println!("{}, " - ".repeat($len)");
    };
}
