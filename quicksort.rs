fn main() {
    let arr = [64, 34, 25, 12, 22, 11, 90, 88, 45, 3];

    println!("Original: {:?}", arr);

    // Recursive Quicksort
    let mut arr1 = arr.clone();
    quicksort_recursive(&mut arr1);
    println!("Recursive Quicksort: {:?}", arr1);

    // Iterative Quicksort
    let mut arr2 = arr.clone();
    quicksort_iterative(&mut arr2);
    println!("Iterative Quicksort:  {:?}", arr2);
}

fn quicksort_recursive(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot_index = partition(arr);
    quicksort_recursive(&mut arr[0..pivot_index]);
    quicksort_recursive(&mut arr[pivot_index + 1..]);
}

fn quicksort_iterative(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    let mut stack = Vec::new();
    stack.push((0, arr.len() - 1));

    while let Some((low, high)) = stack.pop() {
        if low >= high {
            continue;
        }

        let pivot_index = partition_slice(arr, low, high);

        //push the smaller partition first to minimize the stack
        if pivot_index > 0 {
            stack.push((low, pivot_index - 1));
        }
        stack.push((pivot_index + 1, high));
    }
}

//partition helper
fn partition(arr: &mut [i32]) -> usize {
    partition_slice(arr, 0, arr.len() - 1)
}

//general partition used by both
fn partition_slice(arr: &mut [i32], low: usize, high: usize) -> usize {
    let pivot = arr[high];
    let mut i = low;

    for j in low..high {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, high);
    i
}
