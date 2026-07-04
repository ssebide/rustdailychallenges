fn main() {
    let mut arr = [2, 4, 6, 10, 14, 17, 3, 19, 39, 40];

    println!("Before: {:?}", arr);
    merge_sort(&mut arr);
    println!("After:  {:?}", arr);
}

fn merge_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    let mid = arr.len() / 2;

    merge_sort(&mut arr[0..mid]);
    merge_sort(&mut arr[mid..]);

    merge(arr, mid);
}

fn merge(arr: &mut [i32], mid: usize) {
    let left = arr[0..mid].to_vec();
    let right = arr[mid..].to_vec();

    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right.len() {
        arr[k] = right[j];
        j += 1;
        k += 1;
    }
}
