// Functions suggested by the much smarter chatgpt. Used for reference and learning



fn gpt_insertion_sort<T: Ord + Copy>(input: &[T]) -> Vec<T> {
    let mut result = Vec::with_capacity(input.len());

    for &item in input.iter() {
        let pos = result.iter()
                        .position(|&x| x > item)
                        .unwrap_or(result.len());
        result.insert(pos, item);
    }
    result
}

