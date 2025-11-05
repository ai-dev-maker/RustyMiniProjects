pub fn binary_search(array: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = array.len();

    while left < right {
        let mid = (left + right) / 2;

        if array[mid] == target {
            return Some(mid);
        } else if array[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    None
}

fn main() {
    let num = binary_search(&[3, 4, 5, 7, 9, 11], 7).unwrap();
    println!("{}", num);  
    // 3
}
