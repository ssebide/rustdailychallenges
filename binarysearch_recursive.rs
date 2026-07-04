fn main() {
    let a = [2, 3, 6, 9, 10, 17, 20, 24, 26, 30, 31, 38, 40, 50, 55, 67];
    let key = 30;

    match recursive_binary_search(&a, key) {
        Some(i) => println!("Found {} at index {}", key, i),
        None => println!("{} not found", key),
    }
}

// Public API - Clean and simple
fn recursive_binary_search(arr: &[i32], key: i32) -> Option<usize> {
    if arr.is_empty() {
        return None;
    }
    binary_search_helper(arr, key, 0, arr.len() - 1)
}

// Private helper - Does the actual recursion
fn binary_search_helper(arr: &[i32], key: i32, low: usize, high: usize) -> Option<usize> {
    if low > high {
        return None;
    }

    let mid = low + (high - low) / 2;

    if key == arr[mid] {
        Some(mid)
    } else if key < arr[mid] {
        binary_search_helper(arr, key, low, mid - 1)
    } else {
        binary_search_helper(arr, key, mid + 1, high)
    }
}

// fn main() {
//     let mut A = [
//         2, 4, 6, 8, 10, 15, 17, 20, 22, 26, 28, 30, 31, 33, 36, 38, 40, 51, 56, 60,
//     ];
//     A.sort(); //sort the array
//     let key = 28;

//     match binarySearch(&A, key, 0, A.len() - 1) {
//         Some(index) => println!("Found key {} at index {}", key, index),
//         None => println!("Key {} not found", key),
//     }
// }

// fn binarySearch(arr: &[i32], key: i32, low: usize, high: usize) -> Option<usize> {
//     if low > high {
//         return None;
//     }

//     let mid = low + (high - low) / 2;

//     if key == arr[mid] {
//         Some(mid)
//     } else if key < arr[mid] {
//         binarySearch(arr, key, low, mid - 1)
//     } else {
//         binarySearch(arr, key, mid + 1, high)
//     }
// }
