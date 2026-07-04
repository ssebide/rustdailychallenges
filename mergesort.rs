fn main() {
    let a = [2, 4, 6, 10, 14, 17];
    let b = [3, 19, 39, 40];

    let merged = merge_sorted(&a, &b);
    println!("Merged: {:?}", merged);
}

fn merge_sorted(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(a.len() + b.len());
    let mut i = 0;
    let mut j = 0;

    // Merge while both arrays have elements
    while i < a.len() && j < b.len() {
        if a[i] <= b[j] {
            result.push(a[i]);
            i += 1;
        } else {
            result.push(b[j]);
            j += 1;
        }
    }

    // Copy remaining elements from A
    while i < a.len() {
        result.push(a[i]);
        i += 1;
    }

    // Copy remaining elements from B
    while j < b.len() {
        result.push(b[j]);
        j += 1;
    }

    result
}
