
// ---------  Insertion sort -------------

pub fn insertion_sort(input : &Vec<i32>) -> Vec<i32>{
    let mut work_vec  =  input.clone();
    for j in 1..input.len(){
        let key = work_vec[j];
        let mut i : usize  = j;
        while i > 0 && work_vec[i - 1] > key {
            work_vec[i] = work_vec[i- 1];
            i -= 1;
            work_vec[i] = key;
        }
    }
    work_vec
}

// ------- Merge sort ----------------

fn merge(array: &mut [i32], p: usize,q: usize,r : usize ) {
    let n1 = q - p ;
    let n2 = r - q ;

    let mut l_array : Vec<i32> = Vec::with_capacity(n1 +1);
    let mut r_array : Vec<i32> = Vec::with_capacity(n2 +1);

    for i in 0..n1 {
        l_array.push(array[p + i]);
    }
    for j in 0..n2 {
        r_array.push(array[q + j]);
    }
    
    let (mut i, mut j, mut k) = (0, 0, p);
    while i < n1 && j < n2 {
        if l_array[i] <= r_array[j] {
            array[k] = l_array[i];
            i += 1;
        } else {
            array[k] = r_array[j];
            j += 1;
        }
        k += 1;
    }
    while i < n1 {
        array[k] = l_array[i];
        i += 1;
        k += 1;
    }
    while j < n2 {
        array[k] = r_array[j];
        j += 1;
        k += 1;
    }
}

pub fn merge_sort(array : &mut [i32], p : usize, r: usize) {
    if p < r - 1 {
        let q = p + (r - p)/ 2; 
        merge_sort(array, p, q);
        merge_sort(array, q, r);
        merge(array, p, q, r);
    }

}

// ----- Heapsort -----------

fn heapify(array: &mut [i32], idx: usize, heapsize : usize){
    
    let left = |idx:usize| 2*idx +1;
    let right = |idx: usize| 2*idx +2;

    let l: usize = left(idx);
    let r: usize = right(idx);

    let mut largest = idx;

    if l < heapsize && array[l] > array[idx]{
        largest = l;
    }
    if r < heapsize && array[r] > array[largest]{
        largest = r;
    }

    if largest != idx {
        array.swap(idx, largest);
        heapify(array, largest, heapsize);
    }

}

fn max_heap(array: &mut [i32],heapsize : usize ){
    for i in (0..array.len()/2 ).rev(){
        heapify(array, i,heapsize);   
    }
}

pub fn heapsort(array: &mut [i32]){
    let mut heapsize = array.len();
    max_heap(array, heapsize);
        for i in (1..array.len()).rev() {
            array.swap(0,i);
            heapsize -= 1;
            heapify(array, 0, heapsize);
    }
}
