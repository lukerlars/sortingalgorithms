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

const TEST_SIZE:usize = 1000;

fn main() {

    // let insertion_test_vector = create_random_integer_vector(TEST_SIZE);
    // let insertion_start = Instant::now();
    // let insertion_sorted = insertion_sort(&insertion_test_vector);
    // let insertion_duration= insertion_start.elapsed();
    // println!("Insertion sort with a random vector of size {}, took {:?} to sort",TEST_SIZE, insertion_duration);



    // let mut merge_test_vector = create_random_integer_vector(TEST_SIZE);
    // let merge_start = Instant::now();
    // merge_sort(&mut merge_test_vector[..], 0, TEST_SIZE);
    // let merge_duration= merge_start.elapsed();
    // println!("Merge sort with a random vector of size {}, took {:?} to sort",TEST_SIZE, merge_duration);


    let mut heap_test_vector = create_random_integer_vector(TEST_SIZE);
    let heap_start = Instant::now();
    heapsort(&mut heap_test_vector[..]);
    let heap_duration= heap_start.elapsed();
    println!("Heap sort with a random vector of size {}, took {:?} to sort",TEST_SIZE, heap_duration);
    println!("The vector in question {:?}", heap_test_vector);

}
