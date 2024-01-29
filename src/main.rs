#![allow(unused_variables)]
mod sorting_aglorithms;
use sorting_aglorithms::{heapsort,merge_sort,insertion_sort};

use std::time::Instant;
use rand::Rng;
use rand::distributions::Uniform;



fn create_random_integer_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let range = Uniform::new_inclusive(0, 100);

    (0..n).map(|_| rng.sample(&range)).collect()
}


fn main() {

    for test_size in [100_usize, 1000, 10000, 100000].iter() {

    let insertion_test_vector = create_random_integer_vector(*test_size);
    let insertion_start = Instant::now();
    let insertion_sorted = insertion_sort(&insertion_test_vector);
    let insertion_duration= insertion_start.elapsed();
    println!("Insertion sort with a random vector of size {}, took {:?} to sort",test_size, insertion_duration);
    println!("");


    let mut merge_test_vector = create_random_integer_vector(*test_size);
    let merge_start = Instant::now();
    merge_sort(&mut merge_test_vector[..], 0, *test_size);
    let merge_duration= merge_start.elapsed();
    println!("Merge sort with a random vector of size {}, took {:?} to sort",test_size, merge_duration);
    println!("");

    let mut heap_test_vector = create_random_integer_vector(*test_size);
    let heap_start = Instant::now();
    heapsort(&mut heap_test_vector[..]);
    let heap_duration= heap_start.elapsed();
    println!("Heap sort with a random vector of size {}, took {:?} to sort",test_size, heap_duration);
    println!("");
    // println!("The vector in question {:?}", heap_test_vector);
    }

}
