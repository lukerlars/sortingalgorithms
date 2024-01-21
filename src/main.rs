

fn insertion_sort(input : &Vec<i32>) -> Vec<i32>{
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

fn merge_sort(array : &mut [i32], p : usize, r: usize) {
    if p < r - 1 {
        let q = p + (r - p)/ 2;
        merge_sort(array, p, q);
        merge_sort(array, q, r);
        merge(array, p, q, r);
    }

}


fn main() {

    // let some_vec: Vec<i32> = vec![7,6,3,1,4,5];
    // let a = insertion_sort(&some_vec);
    // println!("Is it sorted? {:?}" , a)

    let mut vec = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    let length = vec.len();
    merge_sort(&mut vec[..], 0, length);
    println!("{:?}", vec);



}

