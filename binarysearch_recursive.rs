fn main() {
    let A = [2,4,6,8,10,15,17,20,22,26,28,30,31,33,36,38,40,51,56,60];
    A.sort() //sort the array
    let key = 28;

    match binarySearch(&A, key){
        Some(index) => println!("Found key {} at index {}", key, index),
        None => println!("Key {} not found", key),
    }
}

fn binarySearch(arr: &[i32], key: i32) -> Option<usize>{
    if arr.is_empty(){
        return None;
    }
}