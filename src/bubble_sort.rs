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
