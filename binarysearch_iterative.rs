fn main() {
    let A = [2, 3, 6, 9, 10, 14, 17, 20, 26, 30, 31, 38, 40, 50, 55, 67];
    //A.sort(), sort it before calling the function
    let key = 30;

    match binarySearch(&A, key) {
        Some(index) => println!("Found {} at the index {}", key, index),
        None => println!("{} not found", key),
    }
}

fn binarySearch(arr: &[i32], key: i32) -> Option<usize> {
    if arr.is_empty() {
        return None;
    }

    let mut low = 0;
    let mut high = arr.len() - 1;

    while low <= high {
        let mid = low + (high - low) / 2; //avoids potential overflow

        if key == arr[mid] {
            return Some(mid);
        } else if key < arr[mid] {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }
    None
}
