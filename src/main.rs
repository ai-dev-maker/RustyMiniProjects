mod threads;
mod binary_search;

// use threads::threads;
use binary_search::binary_search;
use binary_search::bubble_sort;

fn main() {
    let mut array = [3, 4, 5, 7, 9, 6, 8];
    println!("original array: {:?}", array);

    bubble_sort(&mut array);

    println!("sorted array: {:?}", array);

    let num = binary_search(&array, 3).unwrap();
    println!("target number: {}", num);
}
