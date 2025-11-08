pub fn bubble_sort(array: &mut [i32]) {
    let n = array.len();

    for _ in 0..n {
        for j in 0..n - 1 {
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
            }
        }
    }
}

pub fn binary_search(array: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = array.len();

    while left < right {
        let mid = (left + right) / 2;

        if array[mid] == target {
            return Some(mid);
        } else if array[mid] > target {
            right = left + 1;
        } else {
            left = right;
        }
    }
    None
}
